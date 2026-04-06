//! Request file schema (`RequestFile`, [`HttpRequestSpec`], [`RequestBody`]).
//!
//! # OpenAPI 3.x alignment (tooling and docs)
//!
//! This crate models a **runnable** HTTP call (concrete URL, template strings, optional JSON/text
//! body). OpenAPI models a **contract** (parameters, `requestBody.content` keyed by media type,
//! schemas). They are not the same, but fields map conceptually as follows:

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::execute::client::{build_client, send_request};
use crate::execute::prepare::expand_http_request_with_overrides;
use crate::execute::types::{ExecutionResult, PreparedRequest};
use crate::stream::events::Event;
use crate::stream::Session;
use nanoid::nanoid;
use nd_constants::REQUEST_FILE_DEFAULT_VERSION;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Response;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::debug;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_version() -> String {
    return nd_constants::REQUEST_FILE_DEFAULT_VERSION.into();
}

fn default_follow_redirects() -> bool {
    return true;
}

fn default_verify_tls() -> bool {
    return true;
}

fn default_deprecated() -> bool {
    return false;
}

// serde helper
fn is_false(b: &bool) -> bool {
    return !*b;
}

/// Root document for a single request file (JSON or YAML).
///
/// `post_script`, when set, is a path string resolved relative to the request file’s directory.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct RequestFile {
    /// Schema version for forward-compatible parsing (default [`nd_constants::DOCUMENT_DEFAULT_VERSION`] if omitted).
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional human-readable label for logs and UIs (backward compatible when omitted).
    #[serde(default)]
    pub name: Option<String>,
    pub request: HttpRequestSpec,
    /// Set only by [`RequestFile::from_file`]; not part of the on-disk format.
    #[serde(skip)]
    #[schemars(skip)]
    pub _path: Option<PathBuf>,
}

impl RequestFile {
    /// Default timeout when `request.timeout_secs` is omitted (seconds).
    pub fn default_timeout_secs() -> u64 {
        return 30;
    }

    /// create JSON Schema ([draft 2020-12](https://json-schema.org/))
    /// tooling.
    pub fn schema() -> serde_json::Value {
        let schema = schemars::schema_for!(Self);
        return serde_json::to_value(&schema).expect("RequestFile JsonSchema serializes to JSON");
    }

    /// Read and deserialize a request file. Extension must be `.json`, `.yaml`, or `.yml`.
    ///
    /// Returns the parsed document and the **parent directory** of `path`, used to resolve
    /// [`RequestFile::post_script`] paths.
    pub fn from_file(path: &Path) -> Result<RequestFile> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let text = std::fs::read_to_string(path)?;

        let mut file: RequestFile = match ext.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&text).map_err(|e| Error::ParseYaml {
                path: path.to_path_buf(),
                source: e,
            })?,
            "json" => serde_json::from_str(&text).map_err(|e| Error::ParseJson {
                path: path.to_path_buf(),
                source: e,
            })?,
            _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
        };

        // Set meta
        file._path = Some(path.to_path_buf());

        debug!(
            path = %path.display(),
            format = %ext,
            name = ?file.name,
            "loaded request file"
        );

        return Ok(file);
    }

    pub async fn execute(&self, session: &mut Session, stream: bool) -> Result<ExecutionResult> {
        return self.execute_with_overrides(session, None, stream).await;
    }

    /// Run the HTTP request after expanding templates. `var_overrides` take precedence over
    /// [`RuntimeEnv`] for `${VAR}` placeholders (not `${!name}` dynamics).
    pub async fn execute_with_overrides(
        &self,
        session: &mut Session,
        var_overrides: Option<&HashMap<String, String>>,
        stream: bool,
    ) -> Result<ExecutionResult> {
        let name = if let Some(name) = &self.name {
            name.clone()
        } else {
            "<unknown>".to_string()
        };

        let path = if let Some(path) = &self._path {
            path.clone()
        } else {
            PathBuf::new()
        };

        debug!(
            path = %path.display(),
            request_name = ?name,
            "execute_request_with_env"
        );

        // build client and start timer
        let prep = self
            .request
            .expand_with_overrides(&session.runtime, var_overrides)?;
        let client = build_client(&self.request)?;

        let start = Instant::now();
        session.emit(|e| Event::HttpRequestStarted {
            request_name: self.name.clone(),
            method: prep.method.to_string(),
            url: prep.url.clone(),
            elapsed: e,
        });

        // call request
        let response = send_request(&client, &prep).await?;

        let status = response.status().as_u16();
        let final_url = response.url().to_string();
        let hdrs = response.headers();
        let content_type = header_content_type(hdrs);
        let content_length = header_content_length(hdrs);

        let mut response_headers: Vec<(String, String)> = Vec::new();
        for (name, value) in hdrs.iter() {
            if let Ok(s) = value.to_str() {
                response_headers.push((name.as_str().to_string(), s.to_string()));
            }
        }

        let body = if stream {
            consume_request_stream(
                session,
                self.name.clone(),
                status,
                final_url.clone(),
                content_type,
                content_length,
                response,
            )
            .await?
        } else {
            response.bytes().await.map_err(Error::Http)?.to_vec()
        };

        let duration = start.elapsed();

        session.emit(|e| Event::HttpResponseCompleted {
            request_name: self.name.clone(),
            status,
            final_url: final_url.clone(),
            elapsed: e,
        });

        debug!(
            status,
            final_url = %final_url,
            ?duration,
            body_len = body.len(),
            "HTTP response received"
        );

        return Ok(ExecutionResult {
            method: prep.method.clone(),
            request_name: self.name.clone(),
            status,
            final_url,
            headers: response_headers,
            body,
            streamed: stream,
            duration,
            // TODO: if a script called this
            initiator_script: None,
            doc: self.clone(),
        });
    }
}

fn header_content_type(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn header_content_length(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

async fn consume_request_stream(
    session: &mut Session,
    request_name: Option<String>,
    status: u16,
    final_url: String,
    content_type: Option<String>,
    content_length: Option<u64>,
    mut response: Response,
) -> Result<Vec<u8>> {
    let stream_id = nanoid!();

    session.emit(|e| Event::HttpResponseStreamStarted {
        id: stream_id.clone(),
        elapsed: e,
        request_name: request_name.clone(),
        status,
        final_url: final_url.clone(),
        content_type,
        content_length,
    });

    let mut body = Vec::new();
    let mut sequence: u64 = 0;
    let mut cumulative: u64 = 0;

    loop {
        let chunk = match response.chunk().await.map_err(Error::Http)? {
            Some(c) => c,
            None => break,
        };

        let data = chunk.to_vec();
        let chunk_len = data.len() as u64;
        body.extend_from_slice(&data);

        cumulative += chunk_len;

        let progress = content_length.and_then(|total| {
            if total == 0 {
                return None;
            }
            Some((cumulative as f32 / total as f32).min(1.0))
        });

        session.emit(|e| Event::HttpResponseStreamChunk {
            id: stream_id.clone(),
            request_name: request_name.clone(),
            sequence,
            data,
            chunk_len,
            bytes_received: cumulative,
            progress,
            elapsed: e,
        });

        // do addition and clamp at u64 bounds
        sequence = sequence.saturating_add(1);
    }

    let total_bytes = body.len() as u64;

    let length_matched = match content_length {
        Some(expected) => total_bytes == expected,
        None => true,
    };

    session.emit(|e| Event::HttpResponseStreamEnded {
        id: stream_id.clone(),
        request_name: request_name.clone(),
        total_bytes,
        elapsed: e,
        expected_total: content_length,
        length_matched,
    });

    return Ok(body);
}

/// HTTP request fields after file parsing; `${VAR}` expansion happens later in the executor.
///
/// OpenAPI-style metadata fields ([`HttpRequestSpec::summary`], etc.) are optional and ignored when
/// sending the request; they exist for documentation, JSON Schema, and tooling.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct HttpRequestSpec {
    /// Case-insensitive method name (e.g. `GET`, `POST`).
    pub method: String,
    pub url: String,
    /// Short summary (OpenAPI `summary`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Long description (OpenAPI `description`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default = "default_deprecated", skip_serializing_if = "is_false")]
    pub deprecated: bool,
    #[serde(default)]
    pub query: HashMap<String, String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<RequestBody>,
    /// Total request timeout in seconds. If omitted, [`RequestFile::default_timeout_secs`] is used.
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default = "default_follow_redirects")]
    pub follow_redirects: bool,
    /// When `false`, TLS certificates are not verified (insecure; for local/dev only).
    #[serde(default = "default_verify_tls")]
    pub verify_tls: bool,
}

impl HttpRequestSpec {
    /// Default `Content-Type` (and parameters) when the request file does not set one in `headers`.
    pub fn get_contenttype_hint(&self) -> Option<&'static str> {
        let body = &self.body;

        if let Some(body) = body {
            return match &body {
                RequestBody::Structured(n) => match n.body_type {
                    RequestBodyKind::Json | RequestBodyKind::Graphql => Some("application/json"),
                    RequestBodyKind::Text | RequestBodyKind::Other => {
                        Some("text/plain; charset=utf-8")
                    }
                    RequestBodyKind::Xml => Some("application/xml; charset=utf-8"),
                    RequestBodyKind::XWwwFormUrlencoded => {
                        Some("application/x-www-form-urlencoded")
                    }
                    RequestBodyKind::FormData => Some("multipart/form-data"),
                    RequestBodyKind::Binary => Some("application/octet-stream"),
                    RequestBodyKind::None => None,
                },
                RequestBody::Json(_) => Some("application/json"),
                RequestBody::Text(_) => Some("text/plain; charset=utf-8"),
            };
        }

        return None;
    }

    pub fn expand(&self, env: &RuntimeEnv) -> Result<PreparedRequest> {
        return self.expand_with_overrides(env, None);
    }

    pub fn expand_with_overrides(
        &self,
        env: &RuntimeEnv,
        overrides: Option<&HashMap<String, String>>,
    ) -> Result<PreparedRequest> {
        return expand_http_request_with_overrides(env, self, overrides);
    }
}

/// Declared format for an explicit [`RequestBody::Structured`] body (drives default `Content-Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RequestBodyKind {
    Json,
    Text,
    Xml,
    Other,
    Graphql,
    #[serde(rename = "x_www_form_urlencoded", alias = "x-www-form-urlencoded")]
    XWwwFormUrlencoded,
    #[serde(rename = "form_data", alias = "form-data")]
    FormData,
    Binary,
    None,
}

/// Explicit body: required `type` plus `content` (shape depends on [`RequestBodyKind`]).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct RequestBodyStructured {
    /// Logical body format (JSON/YAML key `type`).
    #[serde(rename = "type")]
    pub body_type: RequestBodyKind,
    /// For `json` / `graphql`: any JSON value (object, array, or primitive). For `text`, `xml`,
    /// `other`, url-encoded, multipart, and `binary`: a **JSON string** (UTF-8 payload or base64 for
    /// `binary`); `${VAR}` expansion applies to that string.
    pub content: serde_json::Value,
}

/// Request body: shorthand or explicit `type` + `content`.
///
/// `#[serde(untagged)]` tries variants **in order**:
/// 1. [`RequestBody::Structured`] — object with `type` and `content`.
/// 2. [`RequestBody::Text`] — JSON/YAML string (plain text body).
/// 3. [`RequestBody::Json`] — JSON object/array/primitive serialized as the request body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum RequestBody {
    /// `type` + `content` (explicit format and default `Content-Type`).
    Structured(RequestBodyStructured),
    /// Plain string body (non-JSON or literal text).
    Text(String),
    /// JSON object/array/primitive serialized as the request body.
    Json(serde_json::Value),
}

impl Default for RequestFile {
    fn default() -> Self {
        return RequestFile {
            version: REQUEST_FILE_DEFAULT_VERSION.into(),
            name: Some("Httpbin Get".into()),
            request: HttpRequestSpec {
                method: "GET".into(),
                url: "https://httpbin.org/anything".into(),
                summary: None,
                description: None,
                tags: vec![],
                deprecated: false,
                query: HashMap::new(),
                headers: HashMap::new(),
                body: None,
                timeout_secs: None,
                follow_redirects: true,
                verify_tls: true,
            },
            _path: None,
        };
    }
}

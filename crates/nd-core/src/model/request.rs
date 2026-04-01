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
use crate::execute::prepare::expand_http_request;
use crate::execute::types::{ExecutionResult, PreparedRequest};
use nd_constants::REQUEST_FILE_DEFAULT_VERSION;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tracing::debug;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_version() -> String {
    nd_constants::REQUEST_FILE_DEFAULT_VERSION.into()
}

fn default_follow_redirects() -> bool {
    true
}

fn default_verify_tls() -> bool {
    true
}

fn default_deprecated() -> bool {
    false
}

// serde helper
fn is_false(b: &bool) -> bool {
    !*b
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

    pub async fn execute(&self, env: &RuntimeEnv) -> Result<ExecutionResult> {
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
        let prep = self.request.expand(env)?;
        let client = build_client(&self.request)?;
        let start = Instant::now();

        // call request
        let response = send_request(&client, &prep).await?;

        // parse
        let duration = start.elapsed();
        let status = response.status().as_u16();
        let final_url = response.url().to_string();
        let mut response_headers: Vec<(String, String)> = Vec::new();

        for (name, value) in response.headers().iter() {
            if let Ok(s) = value.to_str() {
                response_headers.push((name.as_str().to_string(), s.to_string()));
            }
        }

        let body = response.bytes().await.map_err(Error::Http)?.to_vec();

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
            duration,
            // TODO: if a script called this
            initiator_script: None,
            doc: self.clone(),
        });
    }
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
        return expand_http_request(env, self);
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

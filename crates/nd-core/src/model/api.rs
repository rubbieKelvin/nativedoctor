//! OpenAPI-backed request model.
//!
//! Every request in a nativedoctor workspace is an operation inside an OpenAPI 3.1 document
//! ([`ApiDocument`], loaded with the [`oas3`] crate). OpenAPI describes the API's shape; it has no
//! notion of "the concrete values I want to send right now" (a specific user id, a bearer token,
//! an example body). nativedoctor stores that scratch state in the
//! [`nd_constants::ND_EXTENSION_REQUEST`] (`x-nativedoctor-request`) specification extension on the
//! operation, as [`RequestExample`].

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use oas3::spec::{Operation, PathItem};
use oas3::Spec;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// HTTP method for an operation. `oas3::spec::PathItem` stores operations in named fields (`get`,
/// `post`, ...) rather than a map, so this enum drives explicit field access in [`get_operation`]
/// and [`set_operation`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

impl HttpMethod {
    pub const ALL: [HttpMethod; 8] = [
        HttpMethod::Get,
        HttpMethod::Put,
        HttpMethod::Post,
        HttpMethod::Delete,
        HttpMethod::Options,
        HttpMethod::Head,
        HttpMethod::Patch,
        HttpMethod::Trace,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Put => "PUT",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Head => "HEAD",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Trace => "TRACE",
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Some(HttpMethod::Get),
            "PUT" => Some(HttpMethod::Put),
            "POST" => Some(HttpMethod::Post),
            "DELETE" => Some(HttpMethod::Delete),
            "OPTIONS" => Some(HttpMethod::Options),
            "HEAD" => Some(HttpMethod::Head),
            "PATCH" => Some(HttpMethod::Patch),
            "TRACE" => Some(HttpMethod::Trace),
            _ => None,
        }
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Read the operation stored under `method` on a path item, if any.
pub fn get_operation(item: &PathItem, method: HttpMethod) -> Option<&Operation> {
    match method {
        HttpMethod::Get => item.get.as_ref(),
        HttpMethod::Put => item.put.as_ref(),
        HttpMethod::Post => item.post.as_ref(),
        HttpMethod::Delete => item.delete.as_ref(),
        HttpMethod::Options => item.options.as_ref(),
        HttpMethod::Head => item.head.as_ref(),
        HttpMethod::Patch => item.patch.as_ref(),
        HttpMethod::Trace => item.trace.as_ref(),
    }
}

/// Mutable slot for the operation stored under `method` on a path item (assign or `.get_or_insert`).
pub fn operation_slot_mut(item: &mut PathItem, method: HttpMethod) -> &mut Option<Operation> {
    match method {
        HttpMethod::Get => &mut item.get,
        HttpMethod::Put => &mut item.put,
        HttpMethod::Post => &mut item.post,
        HttpMethod::Delete => &mut item.delete,
        HttpMethod::Options => &mut item.options,
        HttpMethod::Head => &mut item.head,
        HttpMethod::Patch => &mut item.patch,
        HttpMethod::Trace => &mut item.trace,
    }
}

/// Iterate every `(method, operation)` pair actually present on a path item.
pub fn operations(item: &PathItem) -> impl Iterator<Item = (HttpMethod, &Operation)> {
    HttpMethod::ALL
        .into_iter()
        .filter_map(move |m| get_operation(item, m).map(|op| (m, op)))
}

/// Declared format for a [`SendBody::Structured`] body (drives the default `Content-Type`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendBodyKind {
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

/// Explicit body: required `type` plus `content` (shape depends on [`SendBodyKind`]).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendBodyStructured {
    #[serde(rename = "type")]
    pub body_type: SendBodyKind,
    /// For `json` / `graphql`: any JSON value. For `text`, `xml`, `other`, url-encoded, multipart,
    /// and `binary`: a JSON string (UTF-8 payload, or base64 for `binary`); `${VAR}` expansion
    /// applies to that string.
    pub content: serde_json::Value,
}

/// Concrete request body to send.
///
/// `#[serde(untagged)]` tries variants **in order**: [`SendBody::Structured`] (object with `type` +
/// `content`), [`SendBody::Text`] (plain string), then [`SendBody::Json`] (any other JSON value).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SendBody {
    Structured(SendBodyStructured),
    Text(String),
    Json(serde_json::Value),
}

/// The `x-nativedoctor-request` operation extension: concrete values nativedoctor sends for this
/// operation. Everything here may contain `${VAR}` / `${!dynamic}` placeholders, expanded at send
/// time by `nd-engine`.
///
/// Deliberately does not derive `Default`: `bool::default()` is `false`, which would silently
/// disable redirects and TLS verification for a freshly-created request. [`RequestExample::default`]
/// below matches the `#[serde(default = "default_true")]` behavior used when deserializing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestExample {
    /// Values for `{param}` placeholders in the path template.
    #[serde(default)]
    pub path_params: HashMap<String, String>,
    #[serde(default)]
    pub query: HashMap<String, String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<SendBody>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default = "default_true")]
    pub follow_redirects: bool,
    #[serde(default = "default_true")]
    pub verify_tls: bool,
    /// Overrides the document's `servers[0].url` (e.g. to pick a different declared server, or a
    /// literal base URL / `${VAR}`).
    #[serde(default)]
    pub server: Option<String>,
}

fn default_true() -> bool {
    true
}

impl Default for RequestExample {
    fn default() -> Self {
        RequestExample {
            path_params: HashMap::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            timeout_secs: None,
            follow_redirects: true,
            verify_tls: true,
            server: None,
        }
    }
}

impl RequestExample {
    pub fn read_from(op: &Operation) -> Self {
        op.extensions
            .get(nd_constants::ND_EXTENSION_REQUEST)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default()
    }

    pub fn write_into(&self, op: &mut Operation) -> Result<()> {
        let value = serde_json::to_value(self)
            .map_err(|e| Error::Plain(format!("failed to serialize request example: {e}")))?;
        op.extensions
            .insert(nd_constants::ND_EXTENSION_REQUEST.to_string(), value);
        Ok(())
    }
}

/// A fully-resolved request ready for env/template expansion and sending (built from an
/// [`ApiDocument`] operation + its [`RequestExample`]). This is the boundary `nd-engine` expands
/// and sends; it deliberately mirrors what a hand-authored request used to look like before OpenAPI
/// became mandatory, so the expand/send pipeline doesn't need to know about OpenAPI at all.
#[derive(Debug, Clone)]
pub struct ResolvedRequest {
    pub method: HttpMethod,
    /// Base URL + path template with `{param}` substituted; may still contain `${VAR}` placeholders.
    pub url: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<SendBody>,
    pub timeout_secs: Option<u64>,
    pub follow_redirects: bool,
    pub verify_tls: bool,
}

impl ResolvedRequest {
    pub fn default_timeout_secs() -> u64 {
        30
    }

    pub fn get_contenttype_hint(&self) -> Option<&'static str> {
        match &self.body {
            None => None,
            Some(SendBody::Structured(s)) => match s.body_type {
                SendBodyKind::Json | SendBodyKind::Graphql => Some("application/json"),
                SendBodyKind::Text | SendBodyKind::Other => Some("text/plain; charset=utf-8"),
                SendBodyKind::Xml => Some("application/xml; charset=utf-8"),
                SendBodyKind::XWwwFormUrlencoded => Some("application/x-www-form-urlencoded"),
                SendBodyKind::FormData => Some("multipart/form-data"),
                SendBodyKind::Binary => Some("application/octet-stream"),
                SendBodyKind::None => None,
            },
            Some(SendBody::Json(_)) => Some("application/json"),
            Some(SendBody::Text(_)) => Some("text/plain; charset=utf-8"),
        }
    }
}

/// A stable reference to one operation inside a workspace: which OpenAPI document, which path
/// template, which HTTP method. Used by the workspace manifest's sidebar tree and by the desktop
/// app's open tabs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestRef {
    /// Path to the OpenAPI document, relative to the workspace root.
    pub doc: PathBuf,
    pub path: String,
    pub method: HttpMethod,
}

/// A loaded OpenAPI 3.1 document plus the path it was read from (order-preserving via `oas3::Map`,
/// so re-saving keeps a stable, readable diff).
pub struct ApiDocument {
    pub path: PathBuf,
    pub spec: Spec,
}

impl ApiDocument {
    /// Load an existing OpenAPI document (`.yaml`, `.yml`, or `.json`).
    pub fn load(path: &Path) -> Result<Self> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let text = std::fs::read_to_string(path)?;

        let spec = match ext.as_str() {
            "yaml" | "yml" => oas3::from_yaml(&text).map_err(|e| Error::ParseOpenApi {
                path: path.to_path_buf(),
                message: e.to_string(),
            })?,
            "json" => oas3::from_json(&text).map_err(|e| Error::ParseOpenApi {
                path: path.to_path_buf(),
                message: e.to_string(),
            })?,
            _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
        };

        Ok(Self {
            path: path.to_path_buf(),
            spec,
        })
    }

    /// Create a new, empty OpenAPI 3.1 document (one default server) and write it to `path`.
    pub fn create(path: &Path, title: &str, server_url: &str) -> Result<Self> {
        let spec = Spec {
            openapi: "3.1.0".to_string(),
            info: oas3::spec::Info {
                title: title.to_string(),
                summary: None,
                description: None,
                terms_of_service: None,
                version: "0.1.0".to_string(),
                contact: None,
                license: None,
                extensions: Default::default(),
            },
            servers: vec![oas3::spec::Server {
                url: server_url.to_string(),
                description: None,
                variables: Default::default(),
                extensions: Default::default(),
            }],
            paths: None,
            components: None,
            security: Vec::new(),
            tags: Vec::new(),
            webhooks: Default::default(),
            external_docs: None,
            extensions: Default::default(),
        };
        let doc = Self {
            path: path.to_path_buf(),
            spec,
        };
        doc.save()?;
        Ok(doc)
    }

    pub fn save(&self) -> Result<()> {
        let ext = self
            .path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let text = match ext.as_str() {
            "yaml" | "yml" => oas3::to_yaml(&self.spec).map_err(|e| Error::ParseOpenApi {
                path: self.path.clone(),
                message: e.to_string(),
            })?,
            "json" => oas3::to_json(&self.spec).map_err(|e| Error::ParseOpenApi {
                path: self.path.clone(),
                message: e.to_string(),
            })?,
            _ => return Err(Error::UnsupportedFormat(self.path.clone())),
        };

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, text)?;
        Ok(())
    }

    pub fn operation(&self, path: &str, method: HttpMethod) -> Result<&Operation> {
        self.spec
            .paths
            .as_ref()
            .and_then(|paths| paths.get(path))
            .and_then(|item| get_operation(item, method))
            .ok_or_else(|| Error::OperationNotFound {
                doc: self.path.clone(),
                path: path.to_string(),
                method: method.as_str().to_string(),
            })
    }

    /// List every `(path template, method, operation)` triple in this document, sorted by path then
    /// method, for the sidebar tree and CLI listings.
    pub fn list_operations(&self) -> Vec<(String, HttpMethod, &Operation)> {
        let mut out = Vec::new();
        if let Some(paths) = &self.spec.paths {
            for (path, item) in paths.iter() {
                for (method, op) in operations(item) {
                    out.push((path.clone(), method, op));
                }
            }
        }
        out.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.as_str().cmp(b.1.as_str())));
        out
    }

    /// Insert or replace the operation at `path`/`method`, creating the path item if needed.
    pub fn upsert_operation(&mut self, path: &str, method: HttpMethod, operation: Operation) {
        let paths = self.spec.paths.get_or_insert_with(Default::default);
        if !paths.contains_key(path) {
            paths.insert(path.to_string(), PathItem::default());
        }
        let item = paths.get_mut(path).expect("just inserted");
        *operation_slot_mut(item, method) = Some(operation);
    }

    pub fn default_server_url(&self) -> Option<&str> {
        self.spec.servers.first().map(|s| s.url.as_str())
    }

    /// Build a [`ResolvedRequest`] for `path`/`method` from this document's default server (or the
    /// example's [`RequestExample::server`] override) plus the operation's stored example.
    pub fn resolve_request(&self, path: &str, method: HttpMethod) -> Result<ResolvedRequest> {
        let op = self.operation(path, method)?;
        let example = RequestExample::read_from(op);

        let base = example
            .server
            .clone()
            .or_else(|| self.default_server_url().map(str::to_string))
            .unwrap_or_else(|| "${BASE_URL}".to_string());

        let mut resolved_path = path.to_string();
        for (k, v) in &example.path_params {
            resolved_path = resolved_path.replace(&format!("{{{k}}}"), v);
        }

        let url = format!(
            "{}/{}",
            base.trim_end_matches('/'),
            resolved_path.trim_start_matches('/')
        );

        Ok(ResolvedRequest {
            method,
            url,
            query: example.query,
            headers: example.headers,
            body: example.body,
            timeout_secs: example.timeout_secs,
            follow_redirects: example.follow_redirects,
            verify_tls: example.verify_tls,
        })
    }
}

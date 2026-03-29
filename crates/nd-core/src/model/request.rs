//! Request file schema (`RequestFile`, [`HttpRequestSpec`], [`RequestBody`]).
//!
//! # OpenAPI 3.x alignment (tooling and docs)
//!
//! This crate models a **runnable** HTTP call (concrete URL, template strings, optional JSON/text
//! body). OpenAPI models a **contract** (parameters, `requestBody.content` keyed by media type,
//! schemas). They are not the same, but fields map conceptually as follows:
//!
//! | This model | OpenAPI |
//! |------------|---------|
//! | [`HttpRequestSpec::method`] | Operation HTTP method (OpenAPI uses lowercase; we accept any case and normalize when sending). |
//! | [`HttpRequestSpec::url`] | Effective `servers` URL + `path` combined into one template string. |
//! | [`HttpRequestSpec::query`] | `parameters` with `in: query` (we use a flat map of name → value template). |
//! | [`HttpRequestSpec::headers`] | `parameters` with `in: header`. |
//! | [`HttpRequestSpec::body`] | Instance payload. **Shorthand:** a string → text; a JSON object/array → JSON body. **Explicit:** [`RequestBodyStructured`] with `type` + `content` sets the logical format and default `Content-Type` when not overridden in `headers`. |
//! | [`HttpRequestSpec::summary`], [`HttpRequestSpec::description`], [`HttpRequestSpec::tags`], [`HttpRequestSpec::deprecated`] | Subset of OpenAPI `Operation` metadata. Ignored by the executor. |
//! | [`HttpRequestSpec::timeout_secs`], [`HttpRequestSpec::follow_redirects`], [`HttpRequestSpec::verify_tls`] | Client / execution behavior (not part of OpenAPI’s contract model). |
//! | [`RequestFile::name`] | Human-facing label for CLI and logs (optional; distinct from request-level `summary` when both are set). |
//! | [`RequestFile::version`], [`RequestFile::post_script`] | nativedoctor extensions (no OpenAPI equivalent). |
//!
//! # JSON Schema
//!
//! Use [`request_file_json_schema`] for a JSON Schema value describing [`RequestFile`] (e.g. embed
//! under `components.schemas` in a larger OpenAPI document).

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_version() -> String {
    "0.0.0".into()
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

fn is_false(b: &bool) -> bool {
    !*b
}

/// Root document for a single request file (JSON or YAML).
///
/// `post_script`, when set, is a path string resolved relative to the request file’s directory.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct RequestFile {
    /// Schema version for forward-compatible parsing (default `"0.0.0"` if omitted).
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional human-readable label for logs and UIs (backward compatible when omitted).
    #[serde(default)]
    pub name: Option<String>,
    pub request: HttpRequestSpec,
    /// Optional Rhai script path, relative to the directory containing this request file.
    #[serde(default)]
    pub post_script: Option<String>,
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

impl RequestFile {
    /// Default timeout when `request.timeout_secs` is omitted (seconds).
    pub fn default_timeout_secs() -> u64 {
        30
    }
}

/// Default `Content-Type` (and parameters) when the request file does not set one in `headers`.
pub fn content_type_for_body_kind(kind: RequestBodyKind) -> Option<&'static str> {
    match kind {
        RequestBodyKind::Json | RequestBodyKind::Graphql => Some("application/json"),
        RequestBodyKind::Text | RequestBodyKind::Other => Some("text/plain; charset=utf-8"),
        RequestBodyKind::Xml => Some("application/xml; charset=utf-8"),
        RequestBodyKind::XWwwFormUrlencoded => Some("application/x-www-form-urlencoded"),
        RequestBodyKind::FormData => Some("multipart/form-data"),
        RequestBodyKind::Binary => Some("application/octet-stream"),
        RequestBodyKind::None => None,
    }
}

/// JSON Schema ([draft 2020-12](https://json-schema.org/)) for [`RequestFile`], as a JSON value.
///
/// Suitable for `$schema` pointers or embedding under `components.schemas` in OpenAPI-related
/// tooling.
pub fn request_file_json_schema() -> serde_json::Value {
    let schema = schemars::schema_for!(RequestFile);
    serde_json::to_value(&schema).expect("RequestFile JsonSchema serializes to JSON")
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

fn default_version() -> u32 {
    1
}

fn default_follow_redirects() -> bool {
    true
}

fn default_verify_tls() -> bool {
    true
}

/// Root document for a single request file (JSON or YAML).
///
/// `post_script`, when set, is a path string resolved relative to the request file’s directory.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RequestFile {
    /// Schema version for forward-compatible parsing (default `1`).
    #[serde(default = "default_version")]
    pub version: u32,
    /// Optional human-readable label for logs and UIs (backward compatible when omitted).
    #[serde(default)]
    pub name: Option<String>,
    pub request: HttpRequestSpec,
    /// Optional Rhai script path, relative to the directory containing this request file.
    #[serde(default)]
    pub post_script: Option<String>,
}

/// HTTP request fields after file parsing; `${VAR}` expansion happens later in the executor.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HttpRequestSpec {
    /// Case-insensitive method name (e.g. `GET`, `POST`).
    pub method: String,
    pub url: String,
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

/// Request body: either a UTF-8 text payload or a JSON value.
///
/// Serialized with `#[serde(untagged)]` with **`Text` tried first**, then `Json`, so:
/// - JSON `{"body": "plain"}` → text body.
/// - JSON `{"body": {"a": 1}}` or YAML inline map under `body:` → JSON body.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RequestBody {
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

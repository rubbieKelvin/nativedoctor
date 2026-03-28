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

/// Top-level request document (JSON or YAML).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RequestFile {
    #[serde(default = "default_version")]
    pub version: u32,
    pub request: HttpRequestSpec,
    #[serde(default)]
    pub post_script: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct HttpRequestSpec {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub query: HashMap<String, String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<RequestBody>,
    /// Total request timeout in seconds (default: 30).
    #[serde(default)]
    pub timeout_secs: Option<u64>,
    #[serde(default = "default_follow_redirects")]
    pub follow_redirects: bool,
    #[serde(default = "default_verify_tls")]
    pub verify_tls: bool,
}

/// Serialized as either a JSON value (object/array/…) or a plain string (`text` in JSON).
/// YAML can use an inline map/array for JSON bodies; use a string scalar for raw text.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RequestBody {
    /// Plain string body (tried before JSON so scalars stay text).
    Text(String),
    Json(serde_json::Value),
}

impl RequestFile {
    pub fn default_timeout_secs() -> u64 {
        30
    }
}

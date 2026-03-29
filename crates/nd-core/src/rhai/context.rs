//! Immutable HTTP response snapshot passed into Rhai ([`ResponseCtx`]).

use std::collections::HashMap;

use serde_json::Value;

/// Snapshot of the HTTP response (immutable for the duration of a script run).
#[derive(Clone)]
pub struct ResponseCtx {
    pub(crate) status: i64,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body_str: String,
    pub(crate) json_value: Option<Value>,
}

impl ResponseCtx {
    /// Builds context from wire status, a flat header list, and raw body bytes.
    pub fn from_parts(status: u16, headers: &[(String, String)], body: &[u8]) -> Self {
        let header_map: HashMap<String, String> = headers.iter().cloned().collect();
        let body_str = String::from_utf8_lossy(body).to_string();
        let json_value = serde_json::from_slice(body).ok();
        Self {
            status: status as i64,
            headers: header_map,
            body_str,
            json_value,
        }
    }
}

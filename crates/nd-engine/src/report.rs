//! Plain-text and JSON forms of an exchange.

use nd_core::PreparedRequest;
use serde_json::{json, Value};

use crate::session::ResponseHead;

pub const BINARY_NOTICE: &str = "[binary data not shown]";

pub fn is_binary(bytes: &[u8]) -> bool {
    bytes.contains(&0) || std::str::from_utf8(bytes).is_err()
}

/// Formats a complete body for display, keeping key order and number text.
pub fn body_text(bytes: &[u8], pretty: bool) -> String {
    if is_binary(bytes) {
        return BINARY_NOTICE.to_string();
    }
    let text = String::from_utf8_lossy(bytes);
    if !pretty {
        return text.to_string();
    }
    let trimmed = text.trim_start();
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if let Ok(value) = serde_json::from_str::<Value>(&text) {
            if let Ok(formatted) = serde_json::to_string_pretty(&value) {
                return formatted;
            }
        }
    }
    return text.to_string();
}

pub fn pairs_json(pairs: &[(String, String)]) -> Value {
    return Value::Array(
        pairs
            .iter()
            .map(|(name, value)| json!([name, value]))
            .collect(),
    );
}

/// A body as JSON output: parsed if it's JSON, else text, else null for binary.
pub fn body_json(bytes: &[u8]) -> Value {
    if is_binary(bytes) {
        return Value::Null;
    }
    return serde_json::from_slice(bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(bytes).to_string()));
}

pub fn request_json(request: &PreparedRequest, cookie_line: Option<&str>) -> Value {
    let mut headers = request.headers.clone();
    if let Some(cookies) = cookie_line {
        headers.push(("cookie".to_string(), cookies.to_string()));
    }
    return json!({
        "method": request.method,
        "url": request.url.as_str(),
        "headers": pairs_json(&headers),
    });
}

/// The whole exchange. Not redacted: pass it through `Redactor::redact_json` before showing it.
pub fn result_json(
    name: &str,
    request: Value,
    head: &ResponseHead,
    body: &[u8],
    elapsed_ms: Option<u128>,
    failures: &[String],
) -> Value {
    return json!({
        "name": name,
        "request": request,
        "response": {
            "status": head.status,
            "reason": head.reason,
            "version": head.version,
            "headers": pairs_json(&head.headers),
            "body": body_json(body),
            "elapsed_ms": elapsed_ms,
        },
        "assertion_failures": failures,
    });
}

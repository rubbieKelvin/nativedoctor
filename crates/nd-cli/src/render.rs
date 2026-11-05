//! Text formatting for requests and responses.

use colored::Colorize;
use nd_core::{PreparedBody, PreparedRequest};
use serde_json::Value;

use crate::failure::Failure;

pub const BINARY_NOTICE: &str = "[binary data not shown]";

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Print {
    pub request_head: bool,
    pub request_body: bool,
    pub response_head: bool,
    pub response_body: bool,
    pub meta: bool,
}

impl Print {
    pub fn parse(letters: &str) -> Result<Print, Failure> {
        let mut print = Print::default();
        for letter in letters.chars() {
            match letter {
                'H' => print.request_head = true,
                'B' => print.request_body = true,
                'h' => print.response_head = true,
                'b' => print.response_body = true,
                'm' => print.meta = true,
                other => {
                    return Err(Failure::new(format!(
                        "unknown --print letter `{other}`; use H, B, h, b or m"
                    )));
                }
            }
        }
        return Ok(print);
    }
}

pub fn request_head(request: &PreparedRequest, cookies: Option<&str>) -> String {
    let mut target = request.url.path().to_string();
    if let Some(query) = request.url.query() {
        target = format!("{target}?{query}");
    }
    let host = match (request.url.host_str(), request.url.port()) {
        (Some(host), Some(port)) => format!("{host}:{port}"),
        (Some(host), None) => host.to_string(),
        _ => String::new(),
    };

    let mut lines = vec![format!("{} {target}", request.method.bold())];
    lines.push(header_line("host", &host));
    for (name, value) in &request.headers {
        lines.push(header_line(name, value));
    }
    if let Some(cookies) = cookies {
        lines.push(header_line("cookie", cookies));
    }
    return lines.join("\n") + "\n";
}

pub fn request_body(request: &PreparedRequest, pretty: bool) -> Option<String> {
    return match &request.body {
        PreparedBody::None => None,
        PreparedBody::Bytes(bytes) => Some(body_text(bytes, pretty)),
        PreparedBody::Multipart(parts) => {
            let lines: Vec<String> = parts
                .iter()
                .map(|part| match &part.filename {
                    Some(filename) => {
                        format!("{}: file {filename} ({} bytes)", part.name, part.data.len())
                    }
                    None => format!("{}: {}", part.name, String::from_utf8_lossy(&part.data)),
                })
                .collect();
            Some(lines.join("\n"))
        }
    };
}

pub fn response_head(
    version: &str,
    status: u16,
    reason: Option<&str>,
    headers: &[(String, String)],
) -> String {
    let line = format!("{status} {}", reason.unwrap_or(""))
        .trim_end()
        .to_string();
    let line = match status {
        200..=299 => line.green(),
        300..=399 => line.yellow(),
        _ => line.red(),
    };
    let mut lines = vec![format!("{version} {}", line.bold())];
    for (name, value) in headers {
        lines.push(header_line(name, value));
    }
    return lines.join("\n") + "\n";
}

fn header_line(name: &str, value: &str) -> String {
    format!("{}: {value}", name.cyan())
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

pub fn is_binary(bytes: &[u8]) -> bool {
    bytes.contains(&0) || std::str::from_utf8(bytes).is_err()
}

pub fn pairs_json(pairs: &[(String, String)]) -> Value {
    return Value::Array(
        pairs
            .iter()
            .map(|(name, value)| serde_json::json!([name, value]))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_letters() {
        let print = Print::parse("Hb").unwrap();
        assert!(print.request_head && print.response_body);
        assert!(!print.request_body && !print.response_head && !print.meta);
        assert!(Print::parse("x").is_err());
    }

    #[test]
    fn pretty_json_keeps_key_order_and_number_text() {
        let text = body_text(br#"{"z":1.10,"a":12345678901234567890}"#, true);
        assert_eq!(text, "{\n  \"z\": 1.10,\n  \"a\": 12345678901234567890\n}");
    }

    #[test]
    fn plain_text_and_binary() {
        assert_eq!(body_text(b"hello", true), "hello");
        assert_eq!(body_text(b"{not json", true), "{not json");
        assert_eq!(body_text(&[0, 159, 146], true), BINARY_NOTICE);
    }
}

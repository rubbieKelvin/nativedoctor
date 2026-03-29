//! Human-readable formatting of a prepared HTTP request.

use super::client::merge_url_query;
use super::types::PreparedRequest;
use crate::error::Result;

/// Multiline string: request line, headers, blank line, optional body (UTF-8 or “binary” placeholder).
pub fn format_prepared_request(prep: &PreparedRequest) -> Result<String> {
    let url = merge_url_query(&prep.url, &prep.query)?;
    let mut s = format!("{} {}\n", prep.method, url);
    for (k, v) in &prep.headers {
        s.push_str(&format!("{k}: {v}\n"));
    }
    if let Some(b) = &prep.body {
        s.push('\n');
        if let Ok(txt) = std::str::from_utf8(b) {
            s.push_str(txt);
        } else {
            s.push_str(&format!("<{} bytes binary>", b.len()));
        }
        s.push('\n');
    }
    Ok(s)
}

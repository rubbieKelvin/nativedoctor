use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct SendRequestPayload {
    method: String,
    url: String,
    query: Vec<(String, String)>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponsePayload {
    status_code: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
    time_ms: u64,
    error: Option<String>,
}

#[tauri::command]
async fn send_http_request(payload: SendRequestPayload) -> Result<HttpResponsePayload, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let method = match payload.method.to_uppercase().as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        _ => return Err("Invalid HTTP method".to_string()),
    };

    let mut req_builder = client.request(method, &payload.url);

    // Query parameters
    if !payload.query.is_empty() {
        req_builder = req_builder.query(&payload.query);
    }

    // Headers
    let mut headers = HeaderMap::new();
    for (k, v) in payload.headers {
        if !k.is_empty() {
            let name = HeaderName::from_bytes(k.as_bytes())
                .map_err(|_| format!("Invalid header name: {}", k))?;
            let value = HeaderValue::from_str(&v)
                .map_err(|_| format!("Invalid header value for: {}", k))?;
            headers.insert(name, value);
        }
    }
    req_builder = req_builder.headers(headers);

    // Body
    if let Some(body_content) = payload.body {
        if !body_content.is_empty() {
            req_builder = req_builder.body(body_content);
        }
    }

    let start = Instant::now();
    let res = req_builder.send().await.map_err(|e| e.to_string())?;
    let duration = start.elapsed();

    let status_code = res.status().as_u16();
    let status_text = res.status().canonical_reason().unwrap_or("").to_string();

    let mut res_headers = Vec::new();
    for (name, value) in res.headers() {
        res_headers.push((name.to_string(), value.to_str().unwrap_or("").to_string()));
    }

    let body = res.text().await.map_err(|e| e.to_string())?;

    // Try to prettify JSON body
    let pretty_body = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&body) {
        serde_json::to_string_pretty(&json_val).unwrap_or(body)
    } else {
        body
    };

    Ok(HttpResponsePayload {
        status_code,
        status_text,
        headers: res_headers,
        body: pretty_body,
        time_ms: duration.as_millis() as u64,
        error: None,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![send_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

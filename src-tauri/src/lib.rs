// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::collections::HashMap;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Deserialize)]
struct SendHttpRequestPayload {
    method: String,
    url: String,
    #[serde(default)]
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
}

#[derive(serde::Serialize)]
struct HttpResponse {
    status: u16,
    headers: Vec<[String; 2]>,
    body: String,
    duration_ms: u64,
}

#[tauri::command]
async fn send_http_request(payload: SendHttpRequestPayload) -> Result<HttpResponse, String> {
    let method = payload
        .method
        .parse::<reqwest::Method>()
        .map_err(|e| e.to_string())?;
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();

    let mut req = client.request(method, payload.url.as_str());
    if let Some(ref h) = payload.headers {
        for (k, v) in h {
            req = req.header(k.as_str(), v.as_str());
        }
    }
    if let Some(ref b) = payload.body {
        req = req.body(b.clone());
    }

    let res = req.send().await.map_err(|e| e.to_string())?;
    let status = res.status().as_u16();
    let headers: Vec<[String; 2]> = res
        .headers()
        .iter()
        .map(|(n, v)| {
            [
                n.as_str().to_string(),
                v.to_str().unwrap_or("").to_string(),
            ]
        })
        .collect();
    let body = res.text().await.map_err(|e| e.to_string())?;
    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(HttpResponse {
        status,
        headers,
        body,
        duration_ms,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, send_http_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

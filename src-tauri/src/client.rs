//! Outbound HTTP requests. All request types and the send command live here.

use std::collections::HashMap;

use crate::schema::http::HttpResourceFile;

#[derive(serde::Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<[String; 2]>,
    pub body: String,
    pub duration_ms: u64,
}

pub struct HttpClientState {
    pub client: reqwest::Client,
}

#[tauri::command]
pub async fn send_http_request(
    state: tauri::State<'_, HttpClientState>,
    payload: HttpResourceFile,
) -> Result<HttpResponse, String> {
    // let method = payload
    //     .method
    //     .parse::<reqwest::Method>()
    //     .map_err(|e| e.to_string())?;
    // let start = std::time::Instant::now();

    // let mut req = state.client.request(method, payload.url.as_str());
    // if let Some(ref h) = payload.headers {
    //     for (k, v) in h {
    //         req = req.header(k.as_str(), v.as_str());
    //     }
    // }
    // if let Some(ref b) = payload.body {
    //     req = req.body(b.clone());
    // }

    // let res = req.send().await.map_err(|e| e.to_string())?;
    // let status = res.status().as_u16();
    // let headers: Vec<[String; 2]> = res
    //     .headers()
    //     .iter()
    //     .map(|(n, v)| [n.as_str().to_string(), v.to_str().unwrap_or("").to_string()])
    //     .collect();
    // let body = res.text().await.map_err(|e| e.to_string())?;
    // let duration_ms = start.elapsed().as_millis() as u64;

    // Ok(HttpResponse {
    //     status,
    //     headers,
    //     body,
    //     duration_ms,
    // })
    //

    Err(String::from("Not implemented"))
}

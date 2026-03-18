//! Outbound HTTP requests. The send command delegates to HttpResourceFile::call.

use crate::schema::http::{HttpResourceFile, HttpResponse};

pub struct HttpClientState {
    pub client_http1: reqwest::Client,
    pub client_http2: reqwest::Client,
}

#[tauri::command]
pub async fn send_http_request(
    state: tauri::State<'_, HttpClientState>,
    payload: HttpResourceFile,
) -> Result<HttpResponse, String> {
    let use_http2 = payload.settings.use_http2.unwrap_or(true);
    let client = if use_http2 {
        &state.client_http2
    } else {
        &state.client_http1
    };
    payload.call(client).await
}

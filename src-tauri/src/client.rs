//! Outbound HTTP requests. The send command delegates to HttpResourceFile::call.

use crate::schema::http::{HttpResourceFile, HttpResponse};

pub struct HttpClientState {
    pub client: reqwest::Client,
}

#[tauri::command]
pub async fn send_http_request(
    state: tauri::State<'_, HttpClientState>,
    payload: HttpResourceFile,
) -> Result<HttpResponse, String> {
    return payload.call(&state.client).await;
}

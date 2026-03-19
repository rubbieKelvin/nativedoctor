//! Outbound HTTP requests. The send command delegates to HttpResourceFile::call.

use crate::schema::http::{HttpResourceFile, HttpResponse};
use tracing::{error, info};

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

    let method = payload.method.clone();
    let url = payload.url.clone();
    info!(
        method = %method,
        url = %url,
        use_http2 = use_http2,
        "send_http_request: start"
    );

    let result = payload.call(client).await;
    match &result {
        Ok(res) => info!(
            method = %method,
            url = %url,
            status = res.status,
            duration_ms = res.duration_ms,
            "send_http_request: end"
        ),
        Err(e) => error!(
            method = %method,
            url = %url,
            error = %e,
            "send_http_request: failed"
        ),
    }

    result
}

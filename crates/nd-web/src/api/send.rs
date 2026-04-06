use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use base64::Engine as _;
use nd_core::execute::types::ExecutionResult;
use nd_core::model::request::RequestFile;
use nd_core::stream::Session;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::path_sandbox::resolve_allowed_file;
use super::{json_err, AppState};

/// Body for [`post_send`]: execute either the file at `source_path` or an edited `document`.
#[derive(Deserialize)]
pub struct SendHttpBody {
    /// Filesystem path (must fall under configured roots); used for `RequestFile._path` resolution.
    pub source_path: String,
    /// When set, run this document instead of re-reading disk (unsaved editor buffer).
    pub document: Option<serde_json::Value>,
    /// `${VAR}` overrides merged into template expansion (same as CLI).
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

/// Outcome of an HTTP request run from the UI (aligned with [`nd_core::execute::types::ExecutionResult`]).
#[derive(Serialize)]
pub struct HttpSendResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ExecutionResultDto>,
}

/// Serializable HTTP response snapshot for the SPA (body as UTF-8 and/or base64).
#[derive(Serialize)]
pub struct ExecutionResultDto {
    pub status: u16,
    pub duration_ms: u64,
    pub final_url: String,
    pub method: String,
    pub request_name: Option<String>,
    pub headers: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_base64: Option<String>,
    pub body_utf8: bool,
}

pub async fn post_send(
    State(state): State<AppState>,
    Json(body): Json<SendHttpBody>,
) -> Result<Json<HttpSendResponse>, Response> {
    let source = PathBuf::from(&body.source_path);
    let allowed = resolve_allowed_file(&source, state.roots.as_ref()).map_err(|e| {
        let code = if e.contains("outside") {
            StatusCode::FORBIDDEN
        } else {
            StatusCode::NOT_FOUND
        };
        json_err(e, code)
    })?;

    let mut doc = if let Some(v) = body.document {
        let mut d: RequestFile = serde_json::from_value(v).map_err(|e| {
            json_err(
                format!("invalid request document: {e}"),
                StatusCode::BAD_REQUEST,
            )
        })?;
        d._path = Some(allowed.clone());
        d
    } else {
        RequestFile::from_file(&allowed)
            .map_err(|e| json_err(e.to_string(), StatusCode::BAD_REQUEST))?
    };

    doc._path = Some(allowed);

    let overrides = if body.overrides.is_empty() {
        None
    } else {
        Some(&body.overrides)
    };

    if state.no_network_io {
        let prep = doc
            .request
            .expand_with_overrides(state.env.as_ref(), overrides)
            .map_err(|e| json_err(e.to_string(), StatusCode::BAD_REQUEST))?;
        return Ok(Json(HttpSendResponse {
            ok: true,
            error: None,
            result: Some(ExecutionResultDto {
                status: 0,
                duration_ms: 0,
                final_url: prep.url.clone(),
                method: prep.method.as_str().to_string(),
                request_name: doc.name.clone(),
                headers: prep
                    .headers
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                body_text: None,
                body_base64: None,
                body_utf8: true,
            }),
        }));
    }

    let runtime = (*state.env).clone();
    let mut session = Session::new(
        {
            let r = runtime;
            move || Ok(r.clone())
        },
        None,
    )
    .map_err(|e| json_err(e, StatusCode::BAD_REQUEST))?;
    let exec = doc
        .execute_with_overrides(&mut session, overrides, false)
        .await
        .map_err(|e| json_err(e.to_string(), StatusCode::BAD_REQUEST))?;

    Ok(Json(HttpSendResponse {
        ok: true,
        error: None,
        result: Some(execution_to_dto(&exec)),
    }))
}

fn execution_to_dto(exec: &ExecutionResult) -> ExecutionResultDto {
    let body_utf8 = std::str::from_utf8(&exec.body).ok();
    let (body_text, body_base64) = if let Some(s) = body_utf8 {
        (Some(s.to_string()), None)
    } else {
        (
            None,
            Some(base64::engine::general_purpose::STANDARD.encode(&exec.body)),
        )
    };

    let headers = redact_headers(&exec.headers);

    ExecutionResultDto {
        status: exec.status,
        duration_ms: exec.duration.as_millis() as u64,
        final_url: exec.final_url.clone(),
        method: exec.method.as_str().to_string(),
        request_name: exec.request_name.clone(),
        headers,
        body_text,
        body_base64,
        body_utf8: body_utf8.is_some(),
    }
}

/// Hides `Authorization` header values in API responses (mirrors CLI verbose output).
fn redact_headers(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| {
            if k.eq_ignore_ascii_case("authorization") {
                (k.clone(), "<redacted>".into())
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

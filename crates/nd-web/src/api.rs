//! JSON HTTP API consumed by the Vue SPA (`/api/...`) plus the full Axum router including embedded static files.
//!
//! # Routes (`/api` prefix)
//!
//! | Method | Path | Purpose |
//! |--------|------|---------|
//! | GET | `/workspace` | Discovery: valid requests, scripts, skipped invalid files |
//! | GET | `/file?path=` | Raw file text for the editor (sandboxed to roots) |
//! | POST | `/requests/send` | Run a request (from disk or inline JSON document) |
//! | POST | `/scripts/run` | Run a Rhai script under `nd-core` semantics |
//!
//! Non-API paths are served from [`crate::embed`] (SPA fallback to `index.html`).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine as _;
use nd_core::env::RuntimeEnv;
use nd_core::execute::types::ExecutionResult;
use nd_core::model::request::RequestFile;
use nd_core::rhai::{run_rhai_script, Logger, RhaiScriptRunOptions};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

use crate::embed::embedded_static_response;
use crate::path_sandbox::resolve_allowed_file;
use crate::workspace::{build_workspace, WorkspaceSnapshot};

/// Shared server state: canonical workspace roots, runtime env, and dry-run flag.
#[derive(Clone)]
pub struct AppState {
    /// Canonical absolute directories the user allowed; all file access is constrained here.
    pub roots: Arc<Vec<PathBuf>>,
    pub env: Arc<RuntimeEnv>,
    pub no_network_io: bool,
}

/// Query string for [`get_file`]: absolute or workspace-relative path to read as UTF-8 text.
#[derive(Deserialize)]
pub struct FileQuery {
    pub path: String,
}

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

/// Body for [`post_script_run`]: path to a `.rhai` file under the workspace roots.
#[derive(Deserialize)]
pub struct RunScriptBody {
    pub path: String,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
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

/// Result of running a Rhai script, including captured log lines from the Rhai `Logger`.
#[derive(Serialize)]
pub struct ScriptRunResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub logs: Vec<ScriptLogLine>,
}

/// One line from the in-memory Rhai log sink.
#[derive(Serialize)]
pub struct ScriptLogLine {
    pub level: String,
    pub message: String,
    pub elapsed_ms: u128,
}

/// Builds a JSON `{"error": "..."}` response with the given HTTP status.
fn json_err(msg: impl Into<String>, code: StatusCode) -> Response {
    let body = ApiErrorResponse { error: msg.into() };
    (code, Json(body)).into_response()
}

async fn get_workspace(State(state): State<AppState>) -> Result<Json<WorkspaceSnapshot>, Response> {
    let snap = build_workspace(state.roots.as_ref())
        .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(snap))
}

async fn get_file(
    State(state): State<AppState>,
    Query(q): Query<FileQuery>,
) -> Result<([(axum::http::HeaderName, &'static str); 1], String), Response> {
    let p = PathBuf::from(&q.path);
    let allowed = resolve_allowed_file(&p, state.roots.as_ref()).map_err(|e| {
        let code = if e.contains("outside") {
            StatusCode::FORBIDDEN
        } else {
            StatusCode::NOT_FOUND
        };
        json_err(e, code)
    })?;

    let text = std::fs::read_to_string(&allowed)
        .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    return Ok((
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
        text,
    ));
}

async fn post_send(
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

    let exec = doc
        .execute_with_overrides(state.env.as_ref(), overrides)
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

async fn post_script_run(
    State(state): State<AppState>,
    Json(body): Json<RunScriptBody>,
) -> Result<Json<ScriptRunResponse>, Response> {
    let p = PathBuf::from(&body.path);
    let allowed = resolve_allowed_file(&p, state.roots.as_ref()).map_err(|e| {
        let code = if e.contains("outside") {
            StatusCode::FORBIDDEN
        } else {
            StatusCode::NOT_FOUND
        };
        json_err(e, code)
    })?;

    let env = state.env.clone();
    let no_network_io = state.no_network_io;
    let logger = Arc::new(Logger::new());

    let logger_clone = logger.clone();
    let res = tokio::task::spawn_blocking(move || {
        run_rhai_script(
            &allowed,
            env.as_ref(),
            Some(logger_clone),
            RhaiScriptRunOptions { no_network_io },
        )
    })
    .await
    .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let logs: Vec<ScriptLogLine> = logger
        .drain()
        .into_iter()
        .map(|l| ScriptLogLine {
            level: l.level.to_string(),
            message: l.message,
            elapsed_ms: l.elapsed.as_millis(),
        })
        .collect();

    match res {
        Ok(()) => Ok(Json(ScriptRunResponse {
            ok: true,
            error: None,
            logs,
        })),
        Err(e) => Ok(Json(ScriptRunResponse {
            ok: false,
            error: Some(e.to_string()),
            logs,
        })),
    }
}

/// Router for JSON handlers only (no static files). Used in tests and as the `/api` nest target.
pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/workspace", get(get_workspace))
        .route("/file", get(get_file))
        .route("/requests/send", post(post_send))
        .route("/scripts/run", post(post_script_run))
        .with_state(state)
}

/// Full application: nests the JSON API router at `/api` and serves the embedded SPA for all other paths.
pub fn app_router(state: AppState) -> Router {
    let api = api_router(state);

    Router::new()
        .nest("/api", api)
        .fallback(get(|uri: Uri| async move {
            embedded_static_response(uri.path())
        }))
        .layer(CorsLayer::permissive())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use std::sync::Arc;
    use tower::ServiceExt;

    fn test_state(roots: Vec<PathBuf>) -> AppState {
        AppState {
            roots: Arc::new(roots),
            env: Arc::new(RuntimeEnv::new()),
            no_network_io: true,
        }
    }

    #[tokio::test]
    async fn file_outside_roots_is_forbidden() {
        let allowed = tempfile::tempdir().unwrap();
        let other = tempfile::tempdir().unwrap();
        let secret = other.path().join("s.txt");
        std::fs::write(&secret, "x").unwrap();
        let roots = vec![allowed.path().canonicalize().unwrap()];
        let app = api_router(test_state(roots));
        let path = secret.canonicalize().unwrap();
        let uri = format!("/file?path={}", path.to_string_lossy());
        let res = app
            .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn workspace_lists_valid_request_skips_bad_json() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("ok.yaml"),
            "request:\n  method: GET\n  url: https://example.com\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("bad.json"), "not-json").unwrap();
        let roots = vec![dir.path().canonicalize().unwrap()];
        let app = api_router(test_state(roots));
        let res = app
            .oneshot(
                Request::builder()
                    .uri("/workspace")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["requests"][0]["entries"].as_array().unwrap().len(), 1);
        assert_eq!(v["skipped_requests"].as_array().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn app_router_root_serves_embedded_index() {
        let dir = tempfile::tempdir().unwrap();
        let roots = vec![dir.path().canonicalize().unwrap()];
        let app = app_router(test_state(roots));
        let res = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let ct = res
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            ct.contains("text/html") || ct.contains("html"),
            "unexpected Content-Type: {ct}"
        );
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let s = String::from_utf8_lossy(&body);
        assert!(
            s.to_lowercase().contains("html"),
            "expected HTML document from embedded index"
        );
    }
}

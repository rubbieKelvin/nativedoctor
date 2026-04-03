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

use std::path::PathBuf;
use std::sync::Arc;

use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use nd_core::env::RuntimeEnv;
use serde::Serialize;
use tower_http::cors::CorsLayer;

use crate::embed::embedded_static_response;

pub mod file;
pub mod script;
pub mod send;
pub mod workspace;

/// Shared server state: canonical workspace roots, runtime env, and dry-run flag.
#[derive(Clone)]
pub struct AppState {
    /// Canonical absolute directories the user allowed; all file access is constrained here.
    pub roots: Arc<Vec<PathBuf>>,
    pub env: Arc<RuntimeEnv>,
    pub no_network_io: bool,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

/// Builds a JSON `{"error": "..."}` response with the given HTTP status.
pub(crate) fn json_err(msg: impl Into<String>, code: StatusCode) -> Response {
    let body = ApiErrorResponse { error: msg.into() };
    (code, Json(body)).into_response()
}

/// Router for JSON handlers only (no static files). Used in tests and as the `/api` nest target.
pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/workspace", get(workspace::get_workspace))
        .route("/file", get(file::get_file))
        .route("/requests/send", post(send::post_send))
        .route("/scripts/run", post(script::post_script_run))
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
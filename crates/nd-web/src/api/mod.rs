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
//! | GET | `/ws` | WebSocket: send one run command, receive [`nd_core::stream::events::Event`] JSON then `run_complete` |
//!
//! Non-API paths are served from [`crate::embed`] (SPA fallback to `index.html`).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use nd_core::stream::Session;
use serde::Serialize;
use tower_http::cors::CorsLayer;

use crate::embed::embedded_static_response;

pub mod file;
pub mod script;
pub mod send;
pub mod workspace;
pub mod ws;

/// Shared server state: canonical workspace roots, runtime env, and dry-run flag.
#[derive(Clone)]
pub struct AppState {
    /// Canonical absolute directories the user allowed; all file access is constrained here.
    pub roots: Arc<Vec<PathBuf>>,
    pub no_network_io: bool,
    pub env_files: Arc<Vec<PathBuf>>,
    pub persistence_file: Option<PathBuf>,
    /// Keep track of all runs and sessions
    pub sessions: Arc<Mutex<HashMap<String, Arc<Mutex<Session>>>>>,
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
        .route("/ws", get(ws::session_ws))
        .with_state(state)
}

/// Full application: nests the JSON API router at `/api` and serves the embedded SPA for all other paths.
pub fn app_router(state: AppState) -> Router {
    let api = api_router(state);

    return Router::new()
        .nest("/api", api)
        .fallback(get(|uri: Uri| async move {
            embedded_static_response(uri.path())
        }))
        .layer(CorsLayer::permissive());
}

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::Response;
use serde::Deserialize;
use std::path::PathBuf;

use crate::path_sandbox::resolve_allowed_file;
use super::{json_err, AppState};

/// Query string for [`get_file`]: absolute or workspace-relative path to read as UTF-8 text.
#[derive(Deserialize)]
pub struct FileQuery {
    pub path: String,
}

pub async fn get_file(
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

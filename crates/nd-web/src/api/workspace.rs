use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;

use crate::workspace::{build_workspace, WorkspaceSnapshot};
use super::{json_err, AppState};

pub async fn get_workspace(State(state): State<AppState>) -> Result<Json<WorkspaceSnapshot>, Response> {
    let snap = build_workspace(state.roots.as_ref())
        .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(Json(snap))
}

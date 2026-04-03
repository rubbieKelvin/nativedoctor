use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use nd_core::rhai::{run_rhai_script, Logger, RhaiScriptRunOptions};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

use crate::path_sandbox::resolve_allowed_file;
use super::{json_err, AppState};

/// Body for [`post_script_run`]: path to a `.rhai` file under the workspace roots.
#[derive(Deserialize)]
pub struct RunScriptBody {
    pub path: String,
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

pub async fn post_script_run(
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

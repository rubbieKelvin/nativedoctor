use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use nd_core::rhai::{resolver::RhaiScriptRunOptions, run::run_rhai_script};
use nd_core::stream::events::Event;
use nd_core::stream::Session;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::{json_err, AppState};
use crate::path_sandbox::resolve_allowed_file;

/// Body for [`post_script_run`]: path to a `.rhai` file under the workspace roots.
#[derive(Deserialize)]
pub struct RunScriptBody {
    pub path: String,
}

/// Result of running a Rhai script, including log lines from [`Event::Log`] on the run [`Session`].
#[derive(Serialize)]
pub struct ScriptRunResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub logs: Vec<ScriptLogLine>,
}

/// One line from script `log()` mapped from [`Event::Log`].
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

    let session = Arc::new(Mutex::new(
        Session::new(
            {
                let e = (*env).clone();
                move || Ok(e)
            },
            None,
        )
        .map_err(|e| json_err(e, StatusCode::BAD_REQUEST))?,
    ));

    let session_for_thread = session.clone();
    let res = tokio::task::spawn_blocking(move || {
        run_rhai_script(
            &allowed,
            session_for_thread,
            RhaiScriptRunOptions { no_network_io },
        )
    })
    .await
    .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let logs: Vec<ScriptLogLine> = session
        .lock()
        .map_err(|e| json_err(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?
        .events()
        .iter()
        .filter_map(|e| {
            if let Event::Log {
                level,
                message,
                elapsed,
                ..
            } = e
            {
                Some(ScriptLogLine {
                    level: level.to_string(),
                    message: message.clone(),
                    elapsed_ms: elapsed.as_millis(),
                })
            } else {
                None
            }
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

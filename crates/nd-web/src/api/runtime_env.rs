use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Json;
use nd_core::env::RuntimeEnv;
use serde::Serialize;

use super::{json_err, AppState};

#[derive(Serialize)]
pub struct RuntimeEnvSnapshot {
    pub entries: Vec<RuntimeEnvEntryDto>,
}

#[derive(Serialize)]
pub struct RuntimeEnvEntryDto {
    pub key: String,
    pub value: String,
}

#[deprecated]
pub async fn get_runtime_env(
    State(state): State<AppState>,
) -> Result<Json<RuntimeEnvSnapshot>, Response> {
    let env = RuntimeEnv::new()
        .with_env_files(state.env_files.as_ref())
        .map_err(|e| json_err(e.to_string(), StatusCode::BAD_REQUEST))?
        .with_persistence(&state.persistence_file)
        .map_err(|e| json_err(e.to_string(), StatusCode::BAD_REQUEST))?;

    let entries = env
        .entries()
        .into_iter()
        .map(|(key, value)| RuntimeEnvEntryDto { key, value })
        .collect();

    Ok(Json(RuntimeEnvSnapshot { entries }))
}

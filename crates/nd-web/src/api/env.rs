use axum::extract::State;
use axum::Json;
use serde::Serialize;

use super::AppState;

#[derive(Serialize)]
pub struct RuntimeEnvEntryDto {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct RuntimeEnvResponse {
    pub entries: Vec<RuntimeEnvEntryDto>,
}

pub async fn get_runtime_env(State(state): State<AppState>) -> Json<RuntimeEnvResponse> {
    let entries = state
        .env
        .entries()
        .into_iter()
        .map(|(key, value)| RuntimeEnvEntryDto { key, value })
        .collect();
    Json(RuntimeEnvResponse { entries })
}

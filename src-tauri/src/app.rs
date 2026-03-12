use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    pub initial_project_path: Mutex<Option<PathBuf>>,
}

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    return dirs::data_local_dir()
        .map(|p| p.join("nativedoctor"))
        .ok_or_else(|| "Could not resolve app data directory".to_string());
}

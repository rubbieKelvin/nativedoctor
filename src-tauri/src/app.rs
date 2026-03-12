use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    pub initial_project_path: Mutex<Option<PathBuf>>,
}

pub fn get_app_data_dir() -> Result<PathBuf, String> {
    return dirs::data_local_dir()
        .map(|p| p.join("nativedoctor"))
        .ok_or_else(|| "Could not resolve app data directory".to_string());
}

pub fn get_db_path() -> Result<PathBuf, String> {
    let dir = get_app_data_dir()?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    return Ok(dir.join("nativedoctor.db"));
}

pub fn init_db(conn: &rusqlite::Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS recent_projects (
            path TEXT PRIMARY KEY,
            name TEXT,
            opened_at INTEGER NOT NULL
        )",
        [],
    )
    .map_err(|e| e.to_string())?;
    return Ok(());
}

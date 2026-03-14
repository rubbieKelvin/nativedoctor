use crate::app::AppState;
use crate::constants::{
    NATIVE_DOCTOR_PROJECT_FILE, NATIVE_DOCTOR_PROJECT_FILE_PUBLIC_SCHEMA_URL,
    NATIVE_DOCTOR_RESOURCE_FILE_EXTS,
};
use crate::db::get_db_path;
use crate::schema::project::NativedoctorJson;

#[derive(serde::Serialize)]
pub struct RecentProject {
    pub path: String,
    pub name: Option<String>,
    pub opened_at: i64,
}

#[tauri::command]
pub async fn get_recent_projects() -> Result<Vec<RecentProject>, String> {
    let db_path = get_db_path()?;

    tauri::async_runtime::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;

        let mut stmt = conn.prepare(
            "SELECT path, name, opened_at FROM recent_projects ORDER BY opened_at DESC LIMIT 20",
        )

        .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| {
                Ok(RecentProject {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    opened_at: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row.map_err(|e| e.to_string())?);
        }
        Ok(out)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn add_recent_project(path: String, name: Option<String>) -> Result<(), String> {
    let db_path = get_db_path()?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;
    tauri::async_runtime::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT OR REPLACE INTO recent_projects (path, name, opened_at) VALUES (?1, ?2, ?3)",
            rusqlite::params![path, name, ts],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn get_initial_project_path(state: tauri::State<AppState>) -> Option<String> {
    // This is the project path that was passed at args
    // eg: nativedoctor .
    // we'd grab it from mutex
    let mut guard = state.initial_project_path.lock().unwrap();
    return guard.take().and_then(|p| p.to_str().map(String::from));
}

#[tauri::command]
pub fn project_has_nativedoctor(path: String) -> bool {
    return std::path::Path::new(&path)
        .join(NATIVE_DOCTOR_PROJECT_FILE)
        .exists();
}

#[tauri::command]
pub fn read_nativedoctor(path: String) -> Result<NativedoctorJson, String> {
    let file_path = std::path::Path::new(&path).join(NATIVE_DOCTOR_PROJECT_FILE);
    let contents = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    return serde_json::from_str(&contents).map_err(|e| e.to_string());
}

#[tauri::command]
pub fn write_nativedoctor(path: String, payload: NativedoctorJson) -> Result<(), String> {
    let dir = std::path::Path::new(&path);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let file_path = dir.join(NATIVE_DOCTOR_PROJECT_FILE);
    let contents = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
    std::fs::write(file_path, contents).map_err(|e| e.to_string())
}

/// Given the path to a nativedoctor.json file, returns the project root (parent directory).
#[tauri::command]
pub fn get_project_root_from_config_path(config_path: String) -> Result<String, String> {
    let p = std::path::Path::new(&config_path);
    return p
        .parent()
        .and_then(|parent| parent.to_str())
        .map(String::from)
        .ok_or_else(|| "Invalid config path".to_string());
}

/// Discovers resource files (*.request.yaml, *.sequence.yaml) in the project root directory.
/// Only scans the immediate directory, not subdirectories.
#[tauri::command]
pub fn discover_resources(project_path: String) -> Result<Vec<String>, String> {
    let root = std::path::Path::new(&project_path);

    if !root.join(NATIVE_DOCTOR_PROJECT_FILE).exists() {
        return Err(format!("Project has no {}", NATIVE_DOCTOR_PROJECT_FILE));
    }

    let entries = std::fs::read_dir(root).map_err(|e| e.to_string())?;

    let mut resources: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        if NATIVE_DOCTOR_RESOURCE_FILE_EXTS
            .iter()
            .filter(|ext| file_name.ends_with(*ext))
            .count()
            == 1
        {
            resources.push(file_name);
        }
    }

    resources.sort();
    return Ok(resources);
}

#[tauri::command]
pub fn create_project(
    folder_path: String,
    name: String,
    description: String,
) -> Result<String, String> {
    let path = std::path::Path::new(&folder_path);
    let config_path = path.join(NATIVE_DOCTOR_PROJECT_FILE);

    if config_path.exists() {
        return Err("nativedoctor.json already exists in this folder".to_string());
    }

    std::fs::create_dir_all(path).map_err(|e| e.to_string())?;

    let payload = NativedoctorJson {
        name: name,
        description: Some(description),
        metadata: None,
        env_sources: None,
        schema: Some(NATIVE_DOCTOR_PROJECT_FILE_PUBLIC_SCHEMA_URL.to_string()),
    };

    write_nativedoctor(folder_path.clone(), payload)?;
    return Ok(folder_path);
}

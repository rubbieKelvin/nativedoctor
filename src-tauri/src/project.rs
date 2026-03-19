use crate::app::AppState;
use crate::constants::{
    NATIVE_DOCTOR_PROJECT_FILE, NATIVE_DOCTOR_PROJECT_FILE_PUBLIC_SCHEMA_URL,
    NATIVE_DOCTOR_RESOURCE_FILE_EXTS,
};
use crate::db::get_db_path;
use crate::schema::project::NativedoctorJson;
use tracing::{error, info};

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
    info!("get_initial_project_path: taking initial project path (if any)");
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
    let file_path_display = file_path.to_string_lossy().into_owned();
    info!(file_path = %file_path_display, "Reading nativedoctor.json");

    let contents = std::fs::read_to_string(&file_path).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to read nativedoctor.json");
        msg
    })?;

    serde_json::from_str(&contents).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to parse nativedoctor.json");
        msg
    })
}

#[tauri::command]
pub fn write_nativedoctor(path: String, payload: NativedoctorJson) -> Result<(), String> {
    let dir = std::path::Path::new(&path);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let file_path = dir.join(NATIVE_DOCTOR_PROJECT_FILE);
    let file_path_display = file_path.to_string_lossy().into_owned();
    let contents = serde_json::to_string_pretty(&payload).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to serialize nativedoctor.json");
        msg
    })?;

    info!(file_path = %file_path_display, bytes = contents.len(), "Writing nativedoctor.json");
    std::fs::write(file_path, contents).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to write nativedoctor.json");
        msg
    })
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
    let root_display = root.to_string_lossy();
    info!(project_root = %root_display, "Discovering resources");

    if !root.join(NATIVE_DOCTOR_PROJECT_FILE).exists() {
        return Err(format!("Project has no {}", NATIVE_DOCTOR_PROJECT_FILE));
    }

    let entries = std::fs::read_dir(root).map_err(|e| {
        let msg = e.to_string();
        error!(project_root = %root_display, error = %msg, "Failed to read project directory");
        msg
    })?;

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
    info!(
        discovered_resources = resources.len(),
        "Discovered resource files"
    );
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
    info!(
        folder_path = %folder_path,
        config_path = %config_path.to_string_lossy(),
        "Creating new project"
    );

    if config_path.exists() {
        return Err("nativedoctor.json already exists in this folder".to_string());
    }

    std::fs::create_dir_all(path).map_err(|e| {
        let msg = e.to_string();
        error!(error = %msg, folder_path = %folder_path, "Failed to create project directory");
        msg
    })?;

    let payload = NativedoctorJson {
        name: name,
        description: Some(description),
        metadata: None,
        env_sources: None,
        selected_env: None,
        schema: Some(NATIVE_DOCTOR_PROJECT_FILE_PUBLIC_SCHEMA_URL.to_string()),
    };

    write_nativedoctor(folder_path.clone(), payload)?;
    return Ok(folder_path);
}

/// Parses a .env-style file (KEY=VALUE per line, # comments, optional quotes) and returns a map.
fn parse_env_file(contents: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq) = line.find('=') {
            let key = line[..eq].trim();
            let mut value = line[eq + 1..].trim();
            if key.is_empty() {
                continue;
            }
            if (value.starts_with('"') && value.ends_with('"') && value.len() >= 2)
                || (value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2)
            {
                value = &value[1..value.len() - 1];
            }
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}

/// Loads an env file (relative to project root) and returns key-value pairs as a JSON object.
#[tauri::command]
pub fn load_env_file(
    project_path: String,
    relative_path: String,
) -> Result<std::collections::HashMap<String, String>, String> {
    let root = std::path::Path::new(&project_path);
    let root_display = root.to_string_lossy();
    let canonical_root = root.canonicalize().map_err(|e| {
        let msg = e.to_string();
        error!(project_root = %root_display, error = %msg, "Failed to resolve project root");
        msg
    })?;

    let full_path = root.join(&relative_path);
    let canonical = full_path.canonicalize().map_err(|e| {
        let msg = e.to_string();
        error!(
            project_root = %root_display,
            relative_path = %relative_path,
            error = %msg,
            "Failed to resolve env file path"
        );
        msg
    })?;

    if !canonical.starts_with(&canonical_root) {
        return Err("Env file path must be inside the project directory".to_string());
    }

    let contents = std::fs::read_to_string(&canonical).map_err(|e| {
        let msg = e.to_string();
        error!(path = %canonical.to_string_lossy(), error = %msg, "Failed to read env file");
        msg
    })?;

    Ok(parse_env_file(&contents))
}

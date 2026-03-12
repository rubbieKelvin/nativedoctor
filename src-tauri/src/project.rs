use crate::app::AppState;
use crate::db::get_db_path;
use crate::schema::NativedoctorJson;

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
    let mut guard = state.initial_project_path.lock().unwrap();
    return guard.take().and_then(|p| p.to_str().map(String::from));
}

#[tauri::command]
pub fn project_has_nativedoctor(path: String) -> bool {
    return std::path::Path::new(&path)
        .join("nativedoctor.json")
        .exists();
}

#[tauri::command]
pub fn read_nativedoctor(path: String) -> Result<NativedoctorJson, String> {
    let file_path = std::path::Path::new(&path).join("nativedoctor.json");
    let contents = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    return serde_json::from_str(&contents).map_err(|e| e.to_string());
}

#[tauri::command]
pub fn write_nativedoctor(path: String, payload: NativedoctorJson) -> Result<(), String> {
    let dir = std::path::Path::new(&path);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let file_path = dir.join("nativedoctor.json");
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

#[tauri::command]
pub fn create_project(
    folder_path: String,
    name: String,
    description: String,
) -> Result<String, String> {
    let path = std::path::Path::new(&folder_path);
    let config_path = path.join("nativedoctor.json");

    if config_path.exists() {
        return Err("nativedoctor.json already exists in this folder".to_string());
    }

    std::fs::create_dir_all(path).map_err(|e| e.to_string())?;

    let payload = NativedoctorJson {
        name: name,
        description: Some(description),
        metadata: None,
        env_sources: None,
        files: Some(vec![]),
    };

    write_nativedoctor(folder_path.clone(), payload)?;
    return Ok(folder_path);
}

fn sanitize_resource_name(name: &str) -> String {
    let s = name.trim();
    if s.is_empty() {
        return "request".to_string();
    }
    return s
        .chars()
        .map(|c| match c {
            ' ' => '-',
            c if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' => c,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
}

#[tauri::command]
pub fn create_http_resource(project_path: String, name: String) -> Result<String, String> {
    let root = std::path::Path::new(&project_path);
    let config_path = root.join("nativedoctor.json");
    if !config_path.exists() {
        return Err("Project has no nativedoctor.json".to_string());
    }

    let mut config: NativedoctorJson =
        serde_json::from_str(&std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    let base = sanitize_resource_name(&name);
    if base.is_empty() {
        return Err("Invalid resource name".to_string());
    }

    let mut files = config.files.unwrap_or_default();
    let rel_path = "requests";
    let dir = root.join(rel_path);
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let mut candidate = format!("{}/{}.request.json", rel_path, base);
    let mut n = 1u32;
    while root.join(&candidate).exists() {
        n += 1;
        candidate = format!("{}/{}-{}.request.json", rel_path, base, n);
    }

    let request_content = serde_json::json!({
        "type": "http",
        "name": if name.trim().is_empty() { "New request" } else { name.trim() },
        "method": "GET",
        "url": ""
    });
    let full_path = root.join(&candidate);
    std::fs::write(
        full_path,
        serde_json::to_string_pretty(&request_content).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;

    files.push(candidate.clone());
    config.files = Some(files);
    write_nativedoctor(project_path, config)?;

    return Ok(candidate);
}

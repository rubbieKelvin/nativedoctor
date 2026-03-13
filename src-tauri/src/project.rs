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

/// Discovers resource files (*.request.yaml, *.sequence.yaml) in the project root directory.
/// Only scans the immediate directory, not subdirectories.
#[tauri::command]
pub fn discover_resources(project_path: String) -> Result<Vec<String>, String> {
    let root = std::path::Path::new(&project_path);

    if !root.join("nativedoctor.json").exists() {
        return Err("Project has no nativedoctor.json".to_string());
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

        if file_name.ends_with(".request.yaml") || file_name.ends_with(".sequence.yaml") {
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

#[derive(serde::Serialize)]
struct HttpRequestFile {
    #[serde(rename = "type")]
    resource_type: String,
    name: String,
    method: String,
    url: String,
}

/// Key-value pair for params/headers. Used for JSON in/out of read/write_resource_file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePairDto {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
}

/// HTTP resource as stored in .request.yaml. Deserialize from YAML, serialize to JSON for frontend.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RequestResourceFile {
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub params: Option<Vec<KeyValuePairDto>>,
    #[serde(default)]
    pub headers: Option<Vec<KeyValuePairDto>>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub auth: Option<serde_json::Value>,
}

/// Sequence node as stored in .sequence.yaml.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SequenceNodeDto {
    pub id: String,
    pub resource_id: String,
    pub resource_type: String,
}

/// Sequence resource as stored in .sequence.yaml.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SequenceResourceFile {
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flow: Option<Vec<SequenceNodeDto>>,
}

/// Content of a resource file (HTTP or Sequence). Returned as JSON from read_resource_file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ResourceFileContent {
    #[serde(rename = "http")]
    Http(RequestResourceFile),
    #[serde(rename = "sequence")]
    Sequence(SequenceResourceFile),
}

/// Creates a new HTTP request resource file in the project root directory.
/// The file is saved as YAML with the .request.yaml extension.
#[tauri::command]
pub fn create_http_resource(project_path: String, name: String) -> Result<String, String> {
    let root = std::path::Path::new(&project_path);
    let config_path = root.join("nativedoctor.json");
    if !config_path.exists() {
        return Err("Project has no nativedoctor.json".to_string());
    }

    let base = sanitize_resource_name(&name);
    if base.is_empty() {
        return Err("Invalid resource name".to_string());
    }

    let mut candidate = format!("{}.request.yaml", base);
    let mut n = 1u32;
    while root.join(&candidate).exists() {
        n += 1;
        candidate = format!("{}-{}.request.yaml", base, n);
    }

    let request_content = HttpRequestFile {
        resource_type: "http".to_string(),
        name: if name.trim().is_empty() {
            "New request".to_string()
        } else {
            name.trim().to_string()
        },
        method: "GET".to_string(),
        url: String::new(),
    };

    let full_path = root.join(&candidate);
    let yaml_content = serde_yaml::to_string(&request_content).map_err(|e| e.to_string())?;
    std::fs::write(full_path, yaml_content).map_err(|e| e.to_string())?;

    return Ok(candidate);
}

#[derive(serde::Serialize)]
struct SequenceFile {
    #[serde(rename = "type")]
    resource_type: String,
    name: String,
    flow: Vec<serde_json::Value>,
}

/// Creates a new sequence resource file in the project root directory.
#[tauri::command]
pub fn create_sequence_resource(project_path: String, name: String) -> Result<String, String> {
    let root = std::path::Path::new(&project_path);
    let config_path = root.join("nativedoctor.json");
    if !config_path.exists() {
        return Err("Project has no nativedoctor.json".to_string());
    }

    let base = sanitize_resource_name(&name);
    if base.is_empty() {
        return Err("Invalid resource name".to_string());
    }

    let mut candidate = format!("{}.sequence.yaml", base);
    let mut n = 1u32;
    while root.join(&candidate).exists() {
        n += 1;
        candidate = format!("{}-{}.sequence.yaml", base, n);
    }

    let sequence_content = SequenceFile {
        resource_type: "sequence".to_string(),
        name: if name.trim().is_empty() {
            "New sequence".to_string()
        } else {
            name.trim().to_string()
        },
        flow: vec![],
    };

    let full_path = root.join(&candidate);
    let yaml_content = serde_yaml::to_string(&sequence_content).map_err(|e| e.to_string())?;
    std::fs::write(full_path, yaml_content).map_err(|e| e.to_string())?;

    return Ok(candidate);
}

/// Reads a resource file from the project directory, parses YAML, and returns JSON for the frontend.
#[tauri::command]
pub fn read_resource_file(
    project_path: String,
    file_name: String,
) -> Result<ResourceFileContent, String> {
    let root = std::path::Path::new(&project_path);
    let file_path = root.join(&file_name);

    if !file_path.is_file() {
        return Err(format!("File not found: {}", file_name));
    }

    let content = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    serde_yaml::from_str(&content).map_err(|e| e.to_string())
}

/// Writes a resource file to the project directory. Accepts JSON from the frontend and serializes to YAML.
#[tauri::command]
pub fn write_resource_file(
    project_path: String,
    file_name: String,
    payload: ResourceFileContent,
) -> Result<(), String> {
    let root = std::path::Path::new(&project_path);
    let file_path = root.join(&file_name);

    let yaml_content = serde_yaml::to_string(&payload).map_err(|e| e.to_string())?;
    std::fs::write(file_path, yaml_content).map_err(|e| e.to_string())
}

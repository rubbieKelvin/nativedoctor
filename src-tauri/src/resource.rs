use crate::schema::ResourceFileContent;

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
    return serde_yaml::from_str(&content).map_err(|e| e.to_string());
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
    return std::fs::write(file_path, yaml_content).map_err(|e| e.to_string());
}

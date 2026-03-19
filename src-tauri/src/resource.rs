use crate::schema::ResourceFileContent;
use tracing::{error, info};

/// Reads a resource file from the project directory, parses YAML, and returns JSON for the frontend.
#[tauri::command]
pub fn read_resource_file(
    project_path: String,
    file_name: String,
) -> Result<ResourceFileContent, String> {
    let root = std::path::Path::new(&project_path);
    let file_path = root.join(&file_name);
    let file_path_display = file_path.to_string_lossy().into_owned();

    if !file_path.is_file() {
        info!(file_path = %file_path_display, "Resource file not found");
        return Err(format!("File not found: {}", file_name));
    }

    info!(file_path = %file_path_display, "Reading resource file");
    let content = std::fs::read_to_string(&file_path).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to read resource file");
        msg
    })?;

    serde_yaml::from_str(&content).map_err(|e| {
        let msg = e.to_string();
        error!(
            file_path = %file_path_display,
            bytes = content.len(),
            error = %msg,
            "Failed to parse resource YAML"
        );
        msg
    })
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
    let file_path_display = file_path.to_string_lossy().into_owned();

    let yaml_content = serde_yaml::to_string(&payload).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to serialize resource YAML");
        msg
    })?;

    info!(
        file_path = %file_path_display,
        bytes = yaml_content.len(),
        "Writing resource file"
    );
    std::fs::write(file_path, yaml_content).map_err(|e| {
        let msg = e.to_string();
        error!(file_path = %file_path_display, error = %msg, "Failed to write resource YAML");
        msg
    })
}

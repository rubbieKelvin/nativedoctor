use std::fs;
use std::path::PathBuf;

use crate::files::models::{ProjectFiles, _FolderItem, _RequestFileItem};
use nd_core::model::request::RequestFile;

pub mod models;

/// Create a new project folder with `.nativedoctor` manifest, README, and sample request files.
pub fn create_project(root: PathBuf, name: String) -> Result<(), String> {
    if !root.is_dir() {
        return Err("Root should be a directory".into());
    }

    let project_dir = root.join(&name);

    fs::create_dir_all(&project_dir)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;

    let requests_dir = project_dir.join("requests");
    fs::create_dir_all(&requests_dir)
        .map_err(|e| format!("Failed to create requests directory: {}", e))?;

    let request1 = RequestFile {
        name: Some("Test GET Request".into()),
        ..RequestFile::default()
    };
    let request1_path = requests_dir.join("request1.json");
    fs::write(
        &request1_path,
        serde_json::to_string_pretty(&request1)
            .map_err(|e| format!("Failed to serialize request: {}", e))?,
    )
    .map_err(|e| format!("Failed to write request file: {}", e))?;

    let request2 = RequestFile {
        name: Some("Test POST Request".into()),
        request: nd_core::model::request::HttpRequestSpec {
            method: "POST".into(),
            url: "https://httpbin.org/post".into(),
            body: Some(nd_core::model::request::RequestBody::Json(
                serde_json::json!({"key": "value"}),
            )),
            ..RequestFile::default().request
        },
        ..RequestFile::default()
    };
    let request2_path = requests_dir.join("request2.json");
    fs::write(
        &request2_path,
        serde_json::to_string_pretty(&request2)
            .map_err(|e| format!("Failed to serialize request: {}", e))?,
    )
    .map_err(|e| format!("Failed to write request file: {}", e))?;

    let readme = format!("# {}\n\nProject documentation for {}.", name, name);
    let readme_path = project_dir.join("README.md");
    fs::write(&readme_path, readme.as_bytes())
        .map_err(|e| format!("Failed to write README.md: {}", e))?;

    let project_files = ProjectFiles {
        title: name,
        doc: readme_path,
        folders: vec![_FolderItem {
            id: "default".into(),
            name: "Default".into(),
        }],
        requests: vec![
            _RequestFileItem {
                path: PathBuf::from("requests/request1.json"),
                folder: None,
            },
            _RequestFileItem {
                path: PathBuf::from("requests/request2.json"),
                folder: Some("default".into()),
            },
        ],
    };

    fs::write(
        project_dir.join("nativedoctor.yaml"),
        serde_yaml::to_string(&project_files)
            .map_err(|e| format!("Failed to serialize project file: {}", e))?,
    )
    .map_err(|e| format!("Failed to write .nativedoctor file: {}", e))?;

    return Ok(());
}

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct _RequestFileItem {
    pub path: PathBuf,
    pub folder: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct _FolderItem {
    pub id: String,
    pub name: String,
}

// The structure for nativedoctor project files
#[derive(Serialize, Deserialize)]
pub struct ProjectFile {
    pub title: String,
    pub folders: Vec<_FolderItem>,
    pub requests: Vec<_RequestFileItem>,
}
impl ProjectFile {
    pub fn new<S: AsRef<str>>(title: S) -> Self {
        let title = title.as_ref();

        return ProjectFile {
            title: title.to_owned(),
            folders: vec![],
            requests: vec![],
        };
    }

    pub fn load(path: PathBuf) -> Result<Self, String> {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read project file '{}': {}", path.display(), e))?;
        serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse project file '{}': {}", path.display(), e))
    }

    pub fn write(&self, path: PathBuf) -> Result<(), String> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| format!("Failed to serialize project file: {}", e))?;
        std::fs::write(&path, content)
            .map_err(|e| format!("Failed to write project file '{}': {}", path.display(), e))
    }
}

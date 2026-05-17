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
pub struct ProjectFiles {
    pub title: String,
    pub doc: PathBuf,
    pub folders: Vec<_FolderItem>,
    pub requests: Vec<_RequestFileItem>,
}

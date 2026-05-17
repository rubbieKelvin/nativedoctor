use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct _RequestFileItem {
    path: PathBuf,
    folder: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct _FolderItem {
    pub id: String,
    pub name: String,
}

// The structure for nativedoctor project files
#[derive(Serialize, Deserialize)]
pub struct ProjectFiles {
    title: String,
    doc: PathBuf,
    folders: Vec<_FolderItem>,
    requests: Vec<_RequestFileItem>,
}

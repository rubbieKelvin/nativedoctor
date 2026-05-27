use std::path::PathBuf;

use gpui::SharedString;

use crate::files::models::{Folder, ProjectFile};

pub struct Actions;

pub struct OpenRequestFile {
    path: PathBuf,
    name: SharedString,
    folder_id: Option<String>,
}

pub enum Resource {
    Folder(String),
    Request(String),
}

pub struct OpenProject {
    /// This here is the open project
    manifest: Option<ProjectFile>,
    // The rest is kinda like updated values
    name: SharedString,
    path: Option<PathBuf>,
    requests: Vec<OpenRequestFile>,
    folders: Vec<Folder>,
    history: Vec<Actions>,
    deleted_objects: Vec<Resource>,
}

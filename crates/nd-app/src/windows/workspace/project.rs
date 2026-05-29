use std::{collections::HashMap, path::PathBuf};

use gpui::SharedString;
use nd_core::model::{
    project::{ProjectFile, ProjectResource},
    request::RequestFile,
    sequence::SequenceFile,
};
use tracing::warn;

/// Placeholder for undo/redo actions on the project.
pub struct Actions;

#[allow(dead_code)]
pub enum DeletedResource {
    Folder(String),
    Request(String),
}

/// In-memory representation of an open NativeDoctor project.
///
/// Wraps the parsed [`ProjectFile`] manifest together with runtime
/// book keeping such as the file path, history, and deletion tracking.
pub struct OpenProject {
    /// The parsed project manifest
    pub manifest: Option<ProjectFile>,
    /// Project title taken from the manifest
    pub name: SharedString,
    /// Absolute path to the project file on disk
    pub path: PathBuf,
    /// Undo/redo history stack
    #[allow(dead_code)]
    pub history: Vec<Actions>,
    pub loaded_requests: HashMap<String, RequestFile>,
    pub loaded_sequences: HashMap<String, SequenceFile>,
    #[allow(dead_code)]
    pub deleted_objects: Vec<DeletedResource>,
}

impl OpenProject {
    /// Build an [`OpenProject`] from a parsed [`ProjectFile`] manifest.
    ///
    /// `project_path` is the absolute path to the `nativedoctor.yaml` file.
    pub fn from_project_file(project_file: ProjectFile, project_path: PathBuf) -> Self {
        let name: SharedString = project_file.title.clone().into();

        return OpenProject {
            manifest: Some(project_file),
            name,
            path: project_path,
            history: Vec::new(),
            deleted_objects: Vec::new(),
            loaded_requests: HashMap::new(),
            loaded_sequences: HashMap::new(),
        };
    }

    pub fn load_resources(&mut self) {
        if let Some(project) = &self.manifest {
            let resources = project.requests.iter().chain(project.sequences.iter());

            for resource in resources {
                match resource {
                    ProjectResource::Request(path) => {
                        let abs_path = self.path.join(path);
                        let request = match RequestFile::from_file(&abs_path) {
                            Ok(val) => val,
                            Err(e) => {
                                warn!("Could not load resource: {e}");
                                continue;
                            }
                        };
                        self.loaded_requests
                            .insert(path.to_string_lossy().to_string(), request);
                    }
                    ProjectResource::Sequence(path) => {
                        let abs_path = self.path.join(path);
                        let seq = match SequenceFile::from_file(&abs_path) {
                            Ok(val) => val,
                            Err(e) => {
                                warn!("Could not load resource: {e}");
                                continue;
                            }
                        };

                        self.loaded_sequences
                            .insert(path.to_string_lossy().to_string(), seq);
                    }
                    _ => {}
                };
            }
        } else {
            warn!("No project manifest to load from");
        }
    }
}

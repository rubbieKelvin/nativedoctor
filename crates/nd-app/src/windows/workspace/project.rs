use std::{collections::HashMap, path::PathBuf};

use gpui::SharedString;
use nd_core::model::{
    project::{ProjectFile, ProjectResource, ResourceType},
    request::RequestFile,
    sequence::SequenceFile,
};
use tracing::warn;
use uuid::Uuid;

/// Placeholder for undo/redo actions on the project.
pub struct Actions;

#[allow(dead_code)]
pub enum DeletedResource {
    Folder(String),
    Request(String),
}

/// An in-memory scratch file not yet saved to disk.
#[derive(Debug, Clone)]
pub struct ScratchFile {
    #[allow(dead_code)]
    pub id: String,
    pub is_sequence: bool,
    pub request: Option<RequestFile>,
    pub sequence: Option<SequenceFile>,
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
    pub scratch_files: HashMap<String, ScratchFile>,
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
            scratch_files: HashMap::new(),
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

    fn resource_dir(&self, kind: ResourceType) -> PathBuf {
        let resources = match &self.manifest {
            Some(m) => match kind {
                ResourceType::Request => &m.requests,
                ResourceType::Sequence => &m.sequences,
                ResourceType::Environment => return PathBuf::from("environments"),
            },
            None => match kind {
                ResourceType::Request => return PathBuf::from("requests"),
                ResourceType::Sequence => return PathBuf::from("sequences"),
                ResourceType::Environment => return PathBuf::from("environments"),
            },
        };

        for resource in resources {
            let path = match resource {
                ProjectResource::Request(p) | ProjectResource::Sequence(p) => p.clone(),
                ProjectResource::Folder(_, _, children) => {
                    if let Some(dir) = Self::first_leaf_dir(children) {
                        dir
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            if let Some(parent) = path.parent() {
                return parent.to_path_buf();
            }
        }

        return match kind {
            ResourceType::Request => PathBuf::from("requests"),
            ResourceType::Sequence => PathBuf::from("sequences"),
            ResourceType::Environment => PathBuf::from("environments"),
        };
    }

    fn first_leaf_dir(children: &[ProjectResource]) -> Option<PathBuf> {
        for child in children {
            match child {
                ProjectResource::Request(p)
                | ProjectResource::Sequence(p)
                | ProjectResource::Environment(p) => {
                    if let Some(parent) = p.parent() {
                        return Some(parent.to_path_buf());
                    }
                }
                ProjectResource::Folder(_, _, nested) => {
                    if let Some(dir) = Self::first_leaf_dir(nested) {
                        return Some(dir);
                    }
                }
            }
        }
        return None;
    }

    pub fn create_resource_file(
        &mut self,
        filename: &str,
        is_sequence: bool,
    ) -> Result<String, String> {
        let ext = filename
            .rsplit('.')
            .next()
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        if !matches!(ext.as_str(), "json" | "yaml" | "yml") {
            return Err(format!(
                "unsupported extension '.{ext}' — use .json, .yaml, or .yml"
            ));
        }

        let kind = if is_sequence {
            ResourceType::Sequence
        } else {
            ResourceType::Request
        };
        let dir = self.resource_dir(kind);

        let project_root = self
            .path
            .parent()
            .ok_or_else(|| "Cannot determine project root".to_string())?;
        let abs_dir = project_root.join(&dir);
        let abs_path = abs_dir.join(filename);
        let relative_path = dir.join(filename);

        if abs_path.exists() {
            return Err(format!("file already exists: {}", abs_path.display()));
        }

        std::fs::create_dir_all(&abs_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        if is_sequence {
            let seq = SequenceFile {
                version: nd_constants::DOCUMENT_DEFAULT_VERSION.into(),
                name: Some(
                    filename
                        .rsplit('.')
                        .nth(1)
                        .unwrap_or(filename)
                        .to_string(),
                ),
                _path: None,
                groups: Vec::new(),
            };
            let content = match ext.as_str() {
                "json" => serde_json::to_string_pretty(&seq)
                    .map_err(|e| format!("failed to serialize sequence: {e}"))?,
                _ => serde_yaml::to_string(&seq)
                    .map_err(|e| format!("failed to serialize sequence: {e}"))?,
            };
            std::fs::write(&abs_path, content)
                .map_err(|e| format!("failed to write file: {e}"))?;
        } else {
            let req = RequestFile {
                name: Some(
                    filename
                        .rsplit('.')
                        .nth(1)
                        .unwrap_or(filename)
                        .to_string(),
                ),
                ..RequestFile::default()
            };
            let content = match ext.as_str() {
                "json" => serde_json::to_string_pretty(&req)
                    .map_err(|e| format!("failed to serialize request: {e}"))?,
                _ => serde_yaml::to_string(&req)
                    .map_err(|e| format!("failed to serialize request: {e}"))?,
            };
            std::fs::write(&abs_path, content)
                .map_err(|e| format!("failed to write file: {e}"))?;
        }

        if let Some(ref mut manifest) = self.manifest {
            if is_sequence {
                manifest
                    .add_sequence(relative_path.clone())
                    .map_err(|e| format!("failed to update project manifest: {e}"))?;
            } else {
                manifest
                    .add_request(relative_path.clone())
                    .map_err(|e| format!("failed to update project manifest: {e}"))?;
            }
        }

        self.loaded_requests.clear();
        self.loaded_sequences.clear();
        self.load_resources();

        return Ok(relative_path.to_string_lossy().to_string());
    }

    /// Create an in-memory scratch file (not written to disk).
    /// Returns the UUID of the new scratch file.
    pub fn create_scratch_file(&mut self, is_sequence: bool) -> String {
        let id = Uuid::new_v4().to_string();

        if is_sequence {
            let seq = SequenceFile {
                version: nd_constants::DOCUMENT_DEFAULT_VERSION.into(),
                name: Some("Untitled".into()),
                _path: None,
                groups: Vec::new(),
            };
            self.scratch_files.insert(
                id.clone(),
                ScratchFile {
                    id: id.clone(),
                    is_sequence: true,
                    request: None,
                    sequence: Some(seq),
                },
            );
        } else {
            let req = RequestFile {
                name: Some("Untitled".into()),
                ..RequestFile::default()
            };
            self.scratch_files.insert(
                id.clone(),
                ScratchFile {
                    id: id.clone(),
                    is_sequence: false,
                    request: Some(req),
                    sequence: None,
                },
            );
        }

        return id;
    }

    /// Save a scratch file to disk, removing it from the scratch pad and
    /// registering it in the manifest and loaded resources.
    pub fn save_scratch_file(
        &mut self,
        scratch_id: &str,
        filename: &str,
        data: RequestFile,
    ) -> Result<String, String> {
        let ext = filename
            .rsplit('.')
            .next()
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        if !matches!(ext.as_str(), "json" | "yaml" | "yml") {
            return Err(format!(
                "unsupported extension '.{ext}' — use .json, .yaml, or .yml"
            ));
        }

        let dir = self.resource_dir(ResourceType::Request);

        let project_root = self
            .path
            .parent()
            .ok_or_else(|| "Cannot determine project root".to_string())?;
        let abs_dir = project_root.join(&dir);
        let abs_path = abs_dir.join(filename);
        let relative_path = dir.join(filename);

        if abs_path.exists() {
            return Err(format!("file already exists: {}", abs_path.display()));
        }

        std::fs::create_dir_all(&abs_dir)
            .map_err(|e| format!("failed to create directory: {e}"))?;

        let content = match ext.as_str() {
            "json" => serde_json::to_string_pretty(&data)
                .map_err(|e| format!("failed to serialize request: {e}"))?,
            _ => serde_yaml::to_string(&data)
                .map_err(|e| format!("failed to serialize request: {e}"))?,
        };
        std::fs::write(&abs_path, content)
            .map_err(|e| format!("failed to write file: {e}"))?;

        if let Some(ref mut manifest) = self.manifest {
            manifest
                .add_request(relative_path.clone())
                .map_err(|e| format!("failed to update project manifest: {e}"))?;
        }

        // Move from scratch to loaded
        self.scratch_files.remove(scratch_id);

        let rel_str = relative_path.to_string_lossy().to_string();
        let mut loaded_req = data;
        loaded_req._path = Some(abs_path);
        self.loaded_requests.insert(rel_str.clone(), loaded_req);

        return Ok(rel_str);
    }
}

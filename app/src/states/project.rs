use std::path::{Path, PathBuf};

use nd_core::schema::root::LoadedRootObject;

#[derive(Clone, PartialEq)]
pub enum ProjectContentLoadingState {
    None,
    Loading(PathBuf),
    Loaded(LoadedRootObject),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub content: ProjectContentLoadingState,
}

impl ProjectState {
    pub fn new() -> Self {
        return ProjectState {
            content: ProjectContentLoadingState::None,
        };
    }

    pub fn file_name(&self, with_ext: bool) -> Option<String> {
        return match &self.content {
            ProjectContentLoadingState::Loading(path) => {
                let path = Path::new(path);
                let path = match with_ext {
                    true => path.file_name(),
                    false => path.file_stem(),
                };
                path.unwrap().to_str().map(|s| s.to_string())
            }
            ProjectContentLoadingState::Loaded(obj) => {
                let path = obj.path.as_path();
                let path = match with_ext {
                    true => path.file_name(),
                    false => path.file_stem(),
                };
                path.unwrap().to_str().map(|s| s.to_string())
            }
            _ => None,
        };
    }

    pub fn project_name(&self) -> Option<String> {
        return match &self.content {
            ProjectContentLoadingState::Loaded(content) => match &content.schema.project {
                Some(project) => Some(project.name.clone()),
                None => None,
            },
            _ => None,
        };
    }
}

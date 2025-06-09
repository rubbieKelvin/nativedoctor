use std::path::Path;

#[derive(Clone, PartialEq)]
struct ProjectFileData {
    pub path: String,
    pub name: String,
    pub requests: Vec<uuid::Uuid>,
    pub sequences: Vec<uuid::Uuid>,
    pub environments: Vec<String>,
    pub variables: Vec<uuid::Uuid>,
}

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub content: Option<ProjectFileData>,
}

impl ProjectState {
    pub fn new() -> Self {
        return ProjectState { content: None };
    }

    pub fn file_name(&self, with_ext: bool) -> Option<String> {
        return match &self.content {
            Some(content) => {
                let path = Path::new(&content.path);
                let path = match with_ext {
                    true => path.file_name(),
                    false => path.file_stem(),
                };
                path.unwrap().to_str().map(|s| s.to_string())
            }
            None => None,
        };
    }

    pub fn load_file(&self, path: String) -> Result<(), String> {
        return Ok(());
    }
}

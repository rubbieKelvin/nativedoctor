use std::path::Path;

use nd_core::schema::root::RootSchema;

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub path: Option<String>,
    pub content: Option<RootSchema>,
}

impl ProjectState {
    pub fn new() -> Self {
        return ProjectState {
            path: None,
            content: None,
        };
    }

    pub fn file_name(&self, with_ext: bool) -> Option<String> {
        return match &self.path {
            Some(path) => {
                let path = Path::new(path);
                let path = match with_ext {
                    true => path.file_name(),
                    false => path.file_stem(),
                };
                path.unwrap().to_str().map(|s| s.to_string())
            }
            None => None,
        };
    }

    pub fn project_name(&self) -> Option<String> {
        if self.path.is_none() {
            return None;
        }

        let default = self.file_name(false).unwrap();
        let name = match &self.content {
            Some(content) => match &content.project {
                Some(project) => project.name.clone(),
                None => default,
            },
            None => default,
        };
        return Some(name);
    }
}

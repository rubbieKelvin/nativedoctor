use std::path::Path;

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub path: Option<String>,
}

impl ProjectState {
    pub fn new() -> Self {
        return ProjectState { path: None };
    }

    pub fn file_name(&self, with_ext: bool) -> Option<String> {
        return match &self.path {
            Some(path) => {
                let path = Path::new(&path);
                let path = match with_ext {
                    true => path.file_name(),
                    false => path.file_stem(),
                };
                path.unwrap().to_str().map(|s| s.to_string())
            }
            None => None,
        };
    }
}

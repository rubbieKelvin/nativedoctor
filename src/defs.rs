use std::path::Path;

use crate::constants::{EXTENSION_PROJECT_FILE_YAML, EXTENSION_REQUEST_FILE_YAML};

pub enum FileType {
    RequestYamlFile,
    ProjectYamlFile,
}

impl FileType {
    #[allow(unused)]
    pub fn extension(&self) -> &'static str {
        return match self {
            FileType::RequestYamlFile => EXTENSION_REQUEST_FILE_YAML,
            FileType::ProjectYamlFile => EXTENSION_PROJECT_FILE_YAML,
        };
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        let file_name = path.file_name().map(|name| name.to_str()).flatten()?;
        if file_name.ends_with(&format!(".{}", EXTENSION_REQUEST_FILE_YAML)) {
            // is a native doctor request file
            return Some(FileType::RequestYamlFile);
        } else if file_name.ends_with(&format!(".{}", EXTENSION_PROJECT_FILE_YAML)) {
            // project file
            return Some(FileType::ProjectYamlFile);
        }

        return None;
    }
}

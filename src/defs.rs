use std::path::Path;

use crate::constants::EXTENSION_REQUEST_FILE_YAML;

pub enum FileType {
    RequestYamlFile,
}

impl FileType {
    #[allow(unused)]
    pub fn extension(&self) -> &'static str {
        return match self {
            FileType::RequestYamlFile => EXTENSION_REQUEST_FILE_YAML,
        };
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        let file_name = path.file_name().map(|name| name.to_str()).flatten()?;
        if file_name.ends_with(&format!(".{}", EXTENSION_REQUEST_FILE_YAML)) {
            // is a native doctor request file
            return Some(FileType::RequestYamlFile);
        }

        return None;
    }
}

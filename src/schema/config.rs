use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    constants::{CONFIG_FILE_NAME, REQUEST_FOLDER, VERSION},
    utils::sanitize_filename,
};

use super::request::Request;

#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration {
    pub version: (u16, u16, u16),
    pub requests: Vec<String>,
    pub environments: HashMap<String, HashMap<String, String>>,
}

impl BaseConfiguration {
    pub fn new() -> Self {
        return BaseConfiguration {
            version: VERSION,
            requests: vec![],
            environments: HashMap::new(),
        };
    }

    pub fn save(&self, root: &Path) -> Result<(), String> {
        let content = serde_yaml::to_string(self).map_err(|e| e.to_string())?;
        let config_path = Path::new(root.to_str().unwrap()).join(CONFIG_FILE_NAME);
        let mut file = File::create(config_path).map_err(|e| e.to_string())?;

        file.write_all(content.as_bytes()).unwrap();

        return Ok(());
    }

    pub fn add_request(&mut self, request: Request, project_root: &Path) -> Result<(), String> {
        let content = serde_yaml::to_string(&request).map_err(|e| e.to_string())?;
        let path = Path::new(project_root.to_str().unwrap()).join(REQUEST_FOLDER);

        if !path.try_exists().unwrap() {
            fs::create_dir(&path).unwrap();
        }

        let sanitized_name = format!(
            "{}_{}.yaml",
            sanitize_filename(&request.name),
            sanitize_filename(&request.url)
        );

        let path = path.join(&sanitized_name);

        let mut file = File::create(path).map_err(|e| e.to_string())?;

        file.write_all(content.as_bytes()).unwrap();

        // now update the config file
        self.requests.push(sanitized_name);
        return self.save(project_root);
    }
}

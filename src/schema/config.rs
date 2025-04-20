use std::{collections::HashMap, fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

use crate::constants::{CONFIG_FILE_NAME, REQUEST_FOLDER, VERSION};

#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration {
    pub version: (u16, u16, u16),
    pub environments: HashMap<String, HashMap<String, String>>,
}

impl BaseConfiguration {
    pub fn new() -> Self {
        return BaseConfiguration {
            version: VERSION,
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

    pub fn get_requests(&self, project_root: &Path) -> Result<Vec<String>, String> {
        let requests_path = Path::new(project_root.to_str().unwrap()).join(REQUEST_FOLDER);
        if !requests_path.try_exists().expect("Could read project root") {
            return Ok(vec![]);
        }
        let requests_entries = requests_path
            .read_dir()
            .expect("Could not read request directory");

        return Ok(requests_entries
            .map(|e| {
                let entry = e.unwrap();
                return String::from(entry.file_name().to_str().unwrap());
            })
            .collect::<Vec<String>>());
    }
}

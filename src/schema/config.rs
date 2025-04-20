use std::{collections::HashMap, fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

use crate::constants::{CONFIG_FILE_NAME, VERSION};

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
}

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::constants::{CONFIG_FILE_NAME, REQUEST_FOLDER, VERSION};

use super::request::Request;

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

    pub fn get_requests(&self, project_root: &Path) -> Result<HashMap<String, Request>, String> {
        let requests_path = Path::new(project_root.to_str().unwrap()).join(REQUEST_FOLDER);
        if !requests_path.try_exists().expect("Could read project root") {
            return Ok(HashMap::new());
        }
        let requests_entries = requests_path
            .read_dir()
            .expect("Could not read request directory");

        let requests = requests_entries
            .map(|e| {
                let entry = e.unwrap();
                let rpath = entry.path();
                let mut file = File::open(rpath).unwrap();
                let mut buff = String::new();

                file.read_to_string(&mut buff).unwrap();
                let request = serde_yaml::from_str::<Request>(&buff).unwrap();
                return request;
            })
            .collect::<Vec<Request>>();

        let mut result = HashMap::new();

        for req in requests {
            result.insert(req.name.clone(), req);
        }

        return Ok(result);
    }

    /// Load the enviroment variable for the specified enviroment with the default at the lower layer
    pub fn resolved_environment_variables(
        &self,
        name: Option<String>,
    ) -> Result<HashMap<String, String>, String> {
        let mut default = match self.environments.get("*") {
            Some(default_env) => default_env.clone(),
            None => HashMap::new(),
        };

        match name {
            Some(name) => {
                let target_env = self.environments.get(&name);
                match target_env {
                    Some(target_env) => {
                        for (key, value) in target_env.iter() {
                            default.insert(key.clone(), value.clone());
                        }
                    }
                    None => {
                        return Err(format!("Could not get environment variable: {name}"));
                    }
                };
            }
            None => {}
        };

        return Ok(default);
    }
}

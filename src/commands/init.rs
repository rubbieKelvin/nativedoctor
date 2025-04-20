use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::schema::config::BaseConfiguration;

fn initialize_the_directory(path: &Path) -> Result<(), String> {
    // if the path does not exist, let's try to create it
    if !path.try_exists().map_err(|e| e.to_string())? {
        // check if the parent exists so we can create this dir
        if path
            .parent()
            .unwrap()
            .try_exists()
            .map_err(|e| e.to_string())?
        {
            // create the dir
            fs::create_dir(path).map_err(|e| e.to_string())?;
        }
    }

    // check if it's a path
    if !path.is_dir() {
        return Err(String::from("Path should be a directory"));
    }

    // check if it's empty
    let mut has_entry = false;
    for _ in path.read_dir().map_err(|e| e.to_string())? {
        has_entry = true;
        break;
    }

    if has_entry {
        return Err(String::from("Directory must be empty"));
    }
    return Ok(());
}

fn create_configuration_files(path: &Path) -> Result<(), String> {
    let config = BaseConfiguration::new();
    let content = serde_yaml::to_string(&config).map_err(|e| e.to_string())?;

    let config_path = Path::new(path.to_str().unwrap()).join(".dotapi.yaml");
    let mut file = File::create(config_path).map_err(|e| e.to_string())?;

    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())?;

    return Ok(());
}

/// Initialize the dotapi directory
pub fn init(path: &Option<String>) -> Result<(), String> {
    // If path is not provided, we'd create a 'dotapi' folder in the current directory
    // If path is not empty, we'd return with an error
    let path = Path::new(match path {
        Some(path) => path.as_str(),
        None => "./dotapi",
    });

    initialize_the_directory(&path)?;
    create_configuration_files(&path)?;

    return Ok(());
}

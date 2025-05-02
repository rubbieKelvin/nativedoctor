use std::{fs, path::Path};

use crate::{constants::DEFAULT_PROJECT_PATH, schema::config::BaseConfiguration};

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
    return config.save(path);
}

/// Initialize the dotapi directory
pub fn init(path: &Option<String>) -> Result<(), String> {
    // If path is not provided, we'd create a 'dotapi' folder in the current directory
    // If path is not empty, we'd return with an error
    let path = Path::new(match path {
        Some(path) => path.as_str(),
        None => DEFAULT_PROJECT_PATH,
    });

    initialize_the_directory(&path)?;
    create_configuration_files(&path)?;

    return Ok(());
}

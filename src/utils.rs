use std::{
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{
    constants::{CONFIG_FILE_NAME, VERSION},
    schema::config::BaseConfiguration,
};

pub fn get_current_project_config_path() -> Result<PathBuf, String> {
    let cwd = env::current_dir().map_err(|e| e.to_string())?;
    let config_file_path = Path::new(&cwd).join(CONFIG_FILE_NAME);

    if !config_file_path.try_exists().map_err(|e| e.to_string())? {
        return Err(format!("No {CONFIG_FILE_NAME} file in current directory"));
    }
    return Ok(config_file_path);
}

/// check the current dir for the .dotapi.yaml file
/// check if the major and minor version matches...
/// and the bug version is <= the bug fix version of the bin
pub fn load_config(path: &PathBuf) -> Result<BaseConfiguration, String> {
    // load the file into struct
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut config_buffer = String::new();

    file.read_to_string(&mut config_buffer)
        .map_err(|e| e.to_string())?;

    let config =
        serde_yaml::from_str::<BaseConfiguration>(&config_buffer).map_err(|e| e.to_string())?;

    // check major, minor versions
    if config.version.0 != VERSION.0 || config.version.1 != VERSION.1 {
        return Err("Version does not match".to_string());
    }

    // check bug fix version
    if config.version.2 > VERSION.2 {
        return Err("A more recent bugfix version is required".to_string());
    }

    return Ok(config);
}

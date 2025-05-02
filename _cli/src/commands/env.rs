use std::{collections::HashMap, path::Path};

use crate::{
    constants::DEFAULT_ENVIROMENT_NAME,
    utils::{get_current_project_config_path, load_config},
};

pub fn set_enviroment_variable(name: &str, key: &str, value: &str) -> Result<(), String> {
    let config_path = get_current_project_config_path()?;
    let mut config = load_config(&config_path)?;

    let mut env = match config.environments.get(name) {
        Some(env) => env.clone(),
        None => HashMap::new(),
    };

    env.insert(key.to_string(), value.to_string());
    config.environments.insert(name.to_string(), env);
    config.save(Path::new(&config_path).parent().unwrap())?;
    return Ok(());
}

pub fn unset_enviroment_variable(name: &str, key: &str) -> Result<(), String> {
    let config_path = get_current_project_config_path()?;
    let mut config = load_config(&config_path)?;

    let mut env = match config.environments.get(name) {
        Some(env) => env.clone(),
        None => HashMap::new(),
    };

    env.remove(key);
    config.environments.insert(name.to_string(), env);
    config.save(Path::new(&config_path).parent().unwrap())?;
    return Ok(());
}

pub fn delete_env_record(name: &str) -> Result<(), String> {
    if name == DEFAULT_ENVIROMENT_NAME {
        return Err("Cannot delete default environment".to_string());
    }

    let config_path = get_current_project_config_path()?;
    let mut config = load_config(&config_path)?;

    config.environments.remove(name);
    config.save(Path::new(&config_path).parent().unwrap())?;

    return Ok(());
}

pub fn list_env_records(name: Option<String>) -> Result<(), String> {
    let config_path = get_current_project_config_path()?;
    let config = load_config(&config_path)?;

    match name {
        Some(name) => match config.environments.get(&name) {
            Some(env) => {
                println!("Environment: {}", name);
                for (key, value) in env {
                    println!("{}={}", key, value);
                }
                Ok(())
            }
            None => Err(format!("Environment '{}' not found", name)),
        },
        None => {
            // list all environment records and variable counts
            println!("Environment Records:");
            for (name, env) in &config.environments {
                println!("{}: {}", name, env.len());
            }
            Ok(())
        }
    }
}

use super::yaml::{load_api_file, Request, Schema};
use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

struct Runtime {
    schema: Schema,
    filename: String,
    environment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)] // Use untagged so serde tries variants in order
pub enum EnvValue {
    String(String),
    Number(serde_yaml::Number), // Use serde_yaml::Number to preserve integer/float distinction
    Boolean(bool),
    Array(Vec<EnvValue>),              // Nested EnvValue for arrays
    Object(HashMap<String, EnvValue>), // Nested EnvValue for objects
    Null,
}

impl Runtime {
    pub fn new(filename: String, environment: Option<String>) -> Result<Self> {
        // TODO: might need to open this from the cwd the program is runing
        let path = Path::new(&filename);
        let schema = load_api_file(path)?;

        return Ok(Runtime {
            schema,
            filename,
            environment,
        });
    }

    /// This should resolve the env variables by the current environment, and return a clean represengtation of the env
    pub fn build_env(&self) -> HashMap<String, EnvValue> {
        let mut env_vars = HashMap::<String, EnvValue>::new();

        for (key, config) in self.schema.env.iter() {
            // let pick up the value based on the environment
            let resolved_value = match &self.environment {
                Some(env_name) => {
                    // Check if an override exists for the current environment
                    if let Some(override_value) = config.overrides.get(env_name) {
                        override_value
                    } else {
                        // No override for this environment, use the default
                        &config.default
                    }
                }
                None => &config.default,
            };

            // Convert the resolved serde_yaml::Value into our EnvValue enum
            // Using serde_yaml::from_value to deserialize directly
            let env_value: EnvValue = serde_yaml::from_value(resolved_value.clone()).unwrap_or_else(|_| {
                // Handle cases where the Value doesn't match EnvValue variants
                eprintln!("Warning: Failed to convert environment variable '{}' value {:?} to EnvValue. Treating as Null.", key, resolved_value);
                EnvValue::Null // Default to Null on conversion failure
            });

            env_vars.insert(key.clone(), env_value);
        }
        return env_vars;
    }
}

impl Request {
    pub fn build(&self) {}
}

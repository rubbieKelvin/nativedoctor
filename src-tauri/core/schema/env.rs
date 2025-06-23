use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents the definition of a single environment variable.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentVariableSchema {
    pub default: serde_yaml::Value, // Use Value to allow any YAML type
    #[serde(flatten)] // Flatten environment-specific overrides into this struct
    pub overrides: HashMap<String, serde_yaml::Value>,
}

impl EnvironmentVariableSchema {
    #[allow(unused)]
    pub fn new(value: serde_yaml::Value, ovr: Vec<(String, serde_yaml::Value)>) -> Self {
        let mut overrides = HashMap::new();

        for (key, v) in ovr {
            overrides.insert(key, v);
        }

        return EnvironmentVariableSchema {
            default: value,
            overrides,
        };
    }
}

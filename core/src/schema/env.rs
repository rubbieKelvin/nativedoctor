use std::collections::HashMap;

use serde::Deserialize;

/// Represents the definition of a single environment variable.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentVariableSchema {
    pub default: serde_yaml::Value, // Use Value to allow any YAML type
    #[serde(flatten)] // Flatten environment-specific overrides into this struct
    pub overrides: HashMap<String, serde_yaml::Value>,
}
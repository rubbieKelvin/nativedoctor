use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents the definition of a single environment variable.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CallSchema {
    pub main: Vec<String>, // Use Value to allow any YAML type
    #[serde(flatten)] // Flatten environment-specific overrides into this struct
    pub overrides: HashMap<String, Vec<String>>,
}

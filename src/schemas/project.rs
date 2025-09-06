use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

/// Defines the schema for a native doctor project file.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectSchema {
    /// A unique name for the project.
    pub name: String,

    /// The version of the project (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// A brief description of the project (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Defines sequences of requests (flows or scenarios), keyed by name.
    /// Each value is a list of request file paths.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<HashMap<String, Vec<String>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sequence: Option<String>,
}

impl ProjectSchema {
    /// Saves the current project schema instance to the specified file path in YAML format.
    pub fn save_to_path(self, path: &Path) -> Result<(), anyhow::Error> {
        // Ensure the directory exists if the path includes subdirectories.
        // This prevents errors if you're saving to a new directory.
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = File::create(&path)?;
        serde_yaml::to_writer(file, &self)?;

        Ok(())
    }

    /// Reads and deserializes from the specified file path.
    pub fn read_from_path(path: &Path) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let schema: ProjectSchema = serde_yaml::from_reader(file)?;
        Ok(schema)
    }
}

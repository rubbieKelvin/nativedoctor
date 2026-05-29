use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

use super::defaults::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub enum SequenceStep {
    RequestCall,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct SequenceGroup {
    pub name: String,
    pub steps: Vec<SequenceStep>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct SequenceFile {
    /// Schema version for forward-compatible parsing
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(skip)]
    #[schemars(skip)]
    pub _path: Option<PathBuf>,
    pub groups: Vec<SequenceGroup>,
}

impl SequenceFile {
    /// Read and deserialize a sequence file. Extension must be `.json`, `.yaml`, or `.yml`
    pub fn from_file(path: &Path) -> Result<Self> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let text = std::fs::read_to_string(path)?;

        let mut file: SequenceFile = match ext.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&text).map_err(|e| Error::ParseYaml {
                path: path.to_path_buf(),
                source: e,
            })?,
            "json" => serde_json::from_str(&text).map_err(|e| Error::ParseJson {
                path: path.to_path_buf(),
                source: e,
            })?,
            _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
        };

        // Set meta
        file._path = Some(path.to_path_buf());

        debug!(
            path = %path.display(),
            format = %ext,
            name = ?file.name,
            "loaded sequence file"
        );

        return Ok(file);
    }
}

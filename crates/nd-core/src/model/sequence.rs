use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_version() -> String {
    return nd_constants::SEQUENCE_FILE_DEFAULT_VERSION.into();
}

/// One entry in a sequence file: path to a request definition, relative to the sequence file dir.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct SequenceStep {
    /// Path to a request file, relative to the sequence file’s directory.
    pub file: String,
    /// Rhai scripts to run sequentially after the step’s HTTP response and request `post_script`
    /// (paths relative to the sequence file’s directory).
    #[serde(default)]
    pub post_scripts: Vec<String>,
}

/// Sequence document (JSON or YAML). Types live under [`crate::model`]; see [`crate::sequence`] to run.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct SequenceFile {
    /// Document schema version (default [`nd_constants::DOCUMENT_DEFAULT_VERSION`] if omitted).
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional human-readable label (backward compatible when omitted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Key–value pairs merged into [`crate::RuntimeEnv`] when the sequence session starts (after
    /// the process snapshot and, for the CLI, `--env` files).
    #[serde(default)]
    pub initial_variables: HashMap<String, String>,
    /// Request steps to run
    pub steps: Vec<SequenceStep>,
}

/// JSON Schema ([draft 2020-12](https://json-schema.org/)) for [`SequenceFile`], as a JSON value.
pub fn sequence_file_json_schema() -> serde_json::Value {
    let schema = schemars::schema_for!(SequenceFile);
    serde_json::to_value(&schema).expect("SequenceFile JsonSchema serializes to JSON")
}

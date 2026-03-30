use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_version() -> String {
    nd_constants::SEQUENCE_FILE_DEFAULT_VERSION.into()
}

/// One entry in a sequence file: path to a request definition, relative to the sequence file dir.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct SequenceStep {
    /// Path to a request file, relative to the sequence file’s directory.
    pub file: String,
}

/// Sequence document (JSON or YAML). Types live under [`crate::model`]; see [`crate::sequence`] to run.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema)]
pub struct SequenceFile {
    /// Document schema version (default [`nd_constants::DOCUMENT_DEFAULT_VERSION`] if omitted).
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional human-readable label (backward compatible when omitted).
    #[serde(default)]
    pub name: Option<String>,
    /// Request steps to run
    pub steps: Vec<SequenceStep>,
}

/// JSON Schema ([draft 2020-12](https://json-schema.org/)) for [`SequenceFile`], as a JSON value.
pub fn sequence_file_json_schema() -> serde_json::Value {
    let schema = schemars::schema_for!(SequenceFile);
    serde_json::to_value(&schema).expect("SequenceFile JsonSchema serializes to JSON")
}

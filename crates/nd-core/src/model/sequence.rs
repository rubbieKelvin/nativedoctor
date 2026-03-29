use serde::{Deserialize, Serialize};

fn default_version() -> String {
    return "0.0.0".into();
}

/// One entry in a sequence file: path to a request definition, relative to the sequence file dir.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SequenceStep {
    pub file: String,
}

/// Sequence document (JSON or YAML). Types live under [`crate::model`]; see [`crate::sequence`] to run.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SequenceFile {
    #[serde(default = "default_version")]
    pub version: String,
    /// Optional human-readable label (backward compatible when omitted).
    #[serde(default)]
    pub name: Option<String>,
    pub steps: Vec<SequenceStep>,
}

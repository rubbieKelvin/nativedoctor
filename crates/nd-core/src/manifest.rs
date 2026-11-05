//! `nativedoctor.yaml` and `environments/<name>.yaml`.

use serde::{Deserialize, Serialize};

use crate::pairs::Pairs;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_env: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
    #[serde(default, skip_serializing_if = "Pairs::is_empty")]
    pub vars: Pairs,
    #[serde(default, skip_serializing_if = "Defaults::is_empty")]
    pub defaults: Defaults,
}

fn default_version() -> u32 {
    1
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Defaults {
    #[serde(default, skip_serializing_if = "Pairs::is_empty")]
    pub headers: Pairs,
}

impl Defaults {
    fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}

/// Committed file: holds variable values and the names of secrets, never secret values.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(default, skip_serializing_if = "Pairs::is_empty")]
    pub vars: Pairs,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<String>,
}

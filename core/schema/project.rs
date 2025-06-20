use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ProjectDefinationSchema {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub version: Option<String>,
}

impl ProjectDefinationSchema {
    pub fn get_version(&self) -> String {
        let res = self.version.clone().unwrap_or_else(|| "0.0.1".to_string());
        return res;
    }
}
use serde::{Deserialize, Serialize};

/// Represents the configuration section of a request.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct RequestConfigSchema {
    #[serde(default)]
    pub require: Vec<String>, // defaults to empty vec if not present
    pub delay: Option<u32>,   // e.g., "500ms", "1s"
    pub timeout: Option<u32>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: u32,
    #[serde(default)]
    pub class: Option<String>, // where to group this request
    #[serde(default)]
    pub tags: Vec<String>,
}

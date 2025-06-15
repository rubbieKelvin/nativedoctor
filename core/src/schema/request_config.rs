use serde::{Deserialize, Serialize};

/// Represents the configuration section of a request.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RequestConfigSchema {
    #[serde(default)]
    pub depends_on: Vec<String>, // defaults to empty vec if not present
    pub delay: Option<String>,   // e.g., "500ms", "1s"
    pub timeout: Option<String>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: u32,
    #[serde(default)]
    pub folder: Option<String> // where to group this request
}

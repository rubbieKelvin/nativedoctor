use serde::Deserialize;

/// Represents the configuration section of a request.
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RequestConfigSchema {
    #[serde(default)]
    pub depends_on: Vec<String>, // defaults to empty vec if not present
    pub delay: Option<String>,   // e.g., "500ms", "1s"
    pub timeout: Option<String>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: u32,
}

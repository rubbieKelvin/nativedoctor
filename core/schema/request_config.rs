use serde::{Deserialize, Serialize};

/// Represents the configuration section of a request.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct RequestConfigSchema {
    #[serde(default)]
    pub require: Vec<String>, // defaults to empty vec if not present
    pub timeout: Option<u32>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: RetryConfigSchema,
    #[serde(default)]
    pub class: Option<String>, // where to group this request
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct RetryConfigSchema {
    pub count: u32,
    pub delay: u32,
    pub statuscodes: Vec<u8>
}
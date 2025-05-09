use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;

// Use anyhow for simple error handling in the example
use anyhow::{Context, Result};

/// Represents the entire API test file structure.
#[derive(Debug, Deserialize)]
pub struct Schema {
    #[serde(default)] // Make imports optional
    pub imports: Vec<String>,
    #[serde(default)] // Make env optional
    pub env: HashMap<String, EnvironmentVariable>,
    #[serde(default)] // Make requests optional
    pub requests: HashMap<String, Request>,
    #[serde(default)] // Make calls optional
    pub calls: Vec<String>,
}

/// Represents the definition of a single environment variable.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentVariable {
    pub default: serde_yaml::Value, // Use Value to allow any YAML type
    #[serde(flatten)] // Flatten environment-specific overrides into this struct
    pub overrides: HashMap<String, serde_yaml::Value>,
}

/// Represents a single API request definition.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Request {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub config: Option<RequestConfig>, // Optional config block
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>, // Optional headers block
    #[serde(default)]
    pub query: Option<HashMap<String, serde_yaml::Value>>, // Optional query block, values can be complex
    #[serde(default)]
    pub body: Option<RequestBody>, // Optional body block
    #[serde(default)]
    pub script: Option<RequestScript>, // Optional script block
}

/// Represents the configuration section of a request.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestConfig {
    #[serde(default)]
    pub depends_on: Vec<String>, // defaults to empty vec if not present
    pub delay: Option<String>,   // e.g., "500ms", "1s"
    pub timeout: Option<String>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: u32,
}

/// Represents the script section of a request.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RequestScript {
    pub post_request: Option<String>, // Script content as a raw string
                                      // Add other script phases here if needed, e.g., pre_request: Option<String>,
}

// --- Request Body Structs ---

/// Represents the body section of a request.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")] // Use 'type' field to determine which variant to deserialize
pub enum RequestBody {
    #[serde(rename = "json")]
    Json {
        content: serde_yaml::Value, // Use Value to allow any JSON structure (object or array)
    },
    #[serde(rename = "graphql")]
    Graphql {
        query: String,
        variables: Option<serde_yaml::Value>, // GraphQL variables as a JSON-like structure
    },
    #[serde(rename = "xml")]
    Xml {
        content: String, // XML content as a string
    },
    #[serde(rename = "text")]
    Text {
        content: String, // Text content as a string
    },
    #[serde(rename = "form-urlencoded")]
    FormUrlencoded {
        content: String, // Form URL-encoded string
    },
    #[serde(rename = "multipart")]
    Multipart {
        parts: Vec<MultipartPart>, // List of multipart parts
    },
}

/// Represents a single part within a multipart request body.
#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")] // Use 'kind' field to determine field or file
pub enum MultipartPart {
    #[serde(rename = "field")]
    Field { name: String, value: String },
    #[serde(rename = "file")]
    File {
        name: String,
        path: String,
        mime_type: Option<String>, // Optional MIME type
    },
}

// --- Parsing Function ---

/// Parses a YAML string into an Schema struct.
pub fn parse_api_yaml(yaml_content: &str) -> Result<Schema> {
    serde_yaml::from_str(yaml_content).context("Failed to parse API test YAML")
}

/// Parses YAML content from a reader into an Schema struct.
pub fn parse_api_yaml_reader<R: Read>(reader: R) -> Result<Schema> {
    serde_yaml::from_reader(reader).context("Failed to parse API test YAML from reader")
}

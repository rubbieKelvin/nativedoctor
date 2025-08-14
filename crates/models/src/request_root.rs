use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RequestRootModel {
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub config: Option<RequestConfigModel>, // Optional config block
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>, // Optional headers block
    #[serde(default)]
    pub query: Option<Vec<(String, String)>>, // Optional query block, values can be complex
    #[serde(default)]
    pub body: Option<RequestBodyModel>, // Optional body block
}

/// Represents the body section of a request.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")] // Use 'type' field to determine which variant to deserialize
pub enum RequestBodyModel {
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
        parts: Vec<MultipartPartModel>, // List of multipart parts
    },
}

/// Represents a single part within a multipart request body.
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")] // Use 'kind' field to determine field or file
pub enum MultipartPartModel {
    #[serde(rename = "field")]
    Field { name: String, value: String },
    #[serde(rename = "file")]
    File {
        name: String,
        path: String,
        mime_type: Option<String>, // Optional MIME type
    },
}

/// Represents the configuration section of a request.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct RequestConfigModel {
    #[serde(default)]
    pub require: Vec<String>, // defaults to empty vec if not present
    pub timeout: Option<u32>, // e.g., "30s"
    #[serde(default)] // default to 0 if not present
    pub retries: RetryConfigModel,
    #[serde(default)]
    pub class: Option<String>, // where to group this request
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct RetryConfigModel {
    pub count: u32,
    pub delay: u32,
    pub statuscodes: Vec<u8>,
}

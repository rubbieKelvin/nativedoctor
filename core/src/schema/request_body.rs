use serde::Deserialize;

/// Represents the body section of a request.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")] // Use 'type' field to determine which variant to deserialize
pub enum RequestBodySchema {
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
        parts: Vec<MultipartPartSchema>, // List of multipart parts
    },
}

/// Represents a single part within a multipart request body.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")] // Use 'kind' field to determine field or file
pub enum MultipartPartSchema {
    #[serde(rename = "field")]
    Field { name: String, value: String },
    #[serde(rename = "file")]
    File {
        name: String,
        path: String,
        mime_type: Option<String>, // Optional MIME type
    },
}

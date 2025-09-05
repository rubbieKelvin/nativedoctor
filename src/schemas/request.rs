use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-exporting for convenience (might want to support json later too)
pub use serde_yaml::Value as SerdeYamlValue;

/// An enum representing common HTTP methods.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RequestSchema {
    // TODO: update this in the schema 
    // Human readable name for the request
    pub name: String,
    pub method: Method,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RequestConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct RequestConfig {
    #[serde(default)]
    pub require: Vec<String>,
    pub delay: Option<u64>,
    pub timeout: Option<u64>,
    #[serde(default)]
    pub retries: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")] // 'type' field discriminates the enum
pub enum RequestBody {
    // map enum variant to string const
    #[serde(rename = "json")]
    Json(JsonBody),
    #[serde(rename = "graphql")]
    Graphql(GraphqlBody),
    #[serde(rename = "xml")]
    Xml(XmlBody),
    #[serde(rename = "text")]
    Text(TextBody),
    #[serde(rename = "form-urlencoded")]
    FormUrlencoded(FormUrlencodedBody),
    #[serde(rename = "multipart")]
    Multipart(MultipartBody),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct JsonBody {
    pub content: SerdeYamlValue, // Allows any valid yaml
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlBody {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<SerdeYamlValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct XmlBody {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TextBody {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FormUrlencodedBody {
    // TODO: update this in the schema defination
    pub content: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartBody {
    // 'type' field is handled by the parent enum's `#[serde(tag = "type")]`
    pub parts: Vec<MultipartPart>,
}

// Corresponds to `#/definitions/MultipartPart` and its `oneOf` variants
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")] // 'kind' field discriminates the enum
pub enum MultipartPart {
    #[serde(rename = "field")]
    Field(MultipartField),
    #[serde(rename = "file")]
    File(MultipartFile),
}

// Corresponds to `#/definitions/MultipartPart/oneOf/type: object/title: MultipartField`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartField {
    pub name: String,
    pub value: String,
}

// Corresponds to `#/definitions/MultipartPart/oneOf/type: object/title: MultipartFile`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartFile {
    // 'kind' field is handled by the parent enum's `#[serde(tag = "kind")]`
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>
}

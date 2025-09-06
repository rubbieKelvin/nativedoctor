use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, path::PathBuf};

// Re-exporting for convenience (might want to support json later too)
pub use serde_yaml::Value as SerdeYamlValue;

/// An enum representing common HTTP methods.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    #[default]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestSchema {
    // Human readable name for the request
    pub name: String,
    pub method: Method,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RequestConfigSchema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<RequestBodySchema>,
}

impl RequestSchema {
    // Creates an example `RequestSchema` with a default URL and method.
    pub fn example(name: String) -> Self {
        return RequestSchema {
            name,
            url: "https://httpbin.org/get".to_string(),
            method: Method::Get,
            ..Default::default()
        };
    }

    /// Saves the current `RequestSchema` instance to the specified file path in YAML format.
    pub fn save_to_path(self, path: PathBuf) -> Result<(), anyhow::Error> {
        // Ensure the directory exists if the path includes subdirectories.
        // This prevents errors if you're saving to a new directory.
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let file = File::create(&path)?;
        serde_yaml::to_writer(file, &self)?;

        Ok(())
    }

    /// Reads and deserializes a `RequestSchema` from the specified file path.
    pub fn read_from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let file = File::open(&path)?;
        let schema: RequestSchema = serde_yaml::from_reader(file)?;
        Ok(schema)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case", default)]
pub struct RequestConfigSchema {
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
pub enum RequestBodySchema {
    // map enum variant to string const
    #[serde(rename = "json")]
    Json(JsonBodySchema),
    #[serde(rename = "graphql")]
    Graphql(GraphqlBodySchema),
    #[serde(rename = "xml")]
    Xml(XmlBodySchema),
    #[serde(rename = "text")]
    Text(TextBodySchema),
    #[serde(rename = "form-urlencoded")]
    FormUrlencoded(FormUrlencodedBodySchema),
    #[serde(rename = "multipart")]
    Multipart(MultipartBodySchema),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct JsonBodySchema {
    pub content: SerdeYamlValue, // Allows any valid yaml
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GraphqlBodySchema {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<SerdeYamlValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct XmlBodySchema {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct TextBodySchema {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct FormUrlencodedBodySchema {
    pub content: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartBodySchema {
    pub parts: Vec<MultipartPartSchema>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MultipartPartSchema {
    #[serde(rename = "field")]
    Field(MultipartFieldSchema),
    #[serde(rename = "file")]
    File(MultipartFileSchema),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartFieldSchema {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MultipartFileSchema {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

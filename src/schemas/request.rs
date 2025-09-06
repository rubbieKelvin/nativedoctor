use anyhow::Context;
use reqwest::blocking::{Client, RequestBuilder, multipart};
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::{Method as ReqwestMethod, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

// Re-exporting for convenience (might want to support json later too)
pub use serde_yaml::Value as SerdeYamlValue;

/// An enum representing common HTTP methods.
#[derive(Debug, Serialize, Deserialize, Clone, Default, Copy)]
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

impl Into<ReqwestMethod> for Method {
    fn into(self) -> ReqwestMethod {
        return match &self {
            Method::Get => ReqwestMethod::GET,
            Method::Post => ReqwestMethod::POST,
            Method::Put => ReqwestMethod::PUT,
            Method::Delete => ReqwestMethod::DELETE,
            Method::Patch => ReqwestMethod::PATCH,
            Method::Head => ReqwestMethod::HEAD,
            Method::Options => ReqwestMethod::OPTIONS,
            Method::Connect => ReqwestMethod::CONNECT,
            Method::Trace => ReqwestMethod::TRACE,
        };
    }
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
    pub fn save_to_path(self, path: &Path) -> Result<(), anyhow::Error> {
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
    pub fn read_from_path(path: &Path) -> Result<Self, anyhow::Error> {
        let file = File::open(path)?;
        let schema: RequestSchema = serde_yaml::from_reader(file)?;
        Ok(schema)
    }

    /// Builds and returns a `reqwest::blocking::RequestBuilder` from the `RequestSchema`.
    pub fn build_blocking_reqwest(&self, client: &Client) -> Result<RequestBuilder, anyhow::Error> {
        // Create the base request builder with the method and URL
        let url = Url::parse(&self.url).context("Error parsing url {}")?;
        let mut builder = client.request(self.method.into(), url);

        // Add headers if they are present in the schema.
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                builder = builder.header(key, value);
            }
        }

        // Add query parameters if they are present.
        if let Some(query) = &self.query {
            builder = builder.query(query);
        }

        // Handle the request body based on its type.
        if let Some(body) = &self.body {
            builder = match body {
                RequestBodySchema::Json(JsonBodySchema { content }) => builder
                    .header(CONTENT_TYPE, "application/json")
                    .body(content.clone()),
                RequestBodySchema::Graphql(GraphqlBodySchema { query, variables }) => {
                    // GraphQL requests typically use a JSON body with 'query' and 'variables'.
                    let mut graphql_body = HashMap::new();
                    graphql_body.insert("query", JsonValue::from(query.clone()));

                    if let Some(YamlValue::Mapping(variables)) = variables {
                        // let json_vars: JsonValue = serde_json::from_value(
                        //     serde_yaml::to_value(vars).expect("Failed to convert YAML to JSON"),
                        // )
                        // .expect("Failed to deserialize YAML as a valid JSON object");
                        let variable_string =
                            serde_yaml::to_string(&YamlValue::Mapping(variables.clone()))
                                .context("Could not translate gql variable from yaml")?;
                        let json_vars = serde_json::from_str::<JsonValue>(&variable_string)
                            .context("Could not translate gql variable into json")?;

                        graphql_body.insert("variables", json_vars);
                    }

                    builder
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .body(serde_json::to_string(&graphql_body)?)
                }
                RequestBodySchema::Xml(XmlBodySchema { content }) => builder
                    .header(CONTENT_TYPE, "application/xml")
                    .body(content.clone()),
                RequestBodySchema::Text(TextBodySchema { content }) => builder
                    .header(CONTENT_TYPE, "text/plain")
                    .body(content.clone()),
                RequestBodySchema::FormUrlencoded(FormUrlencodedBodySchema { content }) => {
                    builder.form(content)
                }
                RequestBodySchema::Multipart(MultipartBodySchema { parts }) => {
                    let mut form = multipart::Form::new();
                    for part in parts {
                        form = match part {
                            MultipartPartSchema::Field(MultipartFieldSchema { name, value }) => {
                                form.text(name.clone(), value.clone())
                            }
                            MultipartPartSchema::File(MultipartFileSchema {
                                name,
                                path,
                                mime_type,
                            }) => {
                                // Create the multipart file part. reqwest handles the file reading.
                                let file_part = multipart::Part::file(PathBuf::from(path.clone()))
                                    .expect("Failed to create multipart file part.");

                                // Set the MIME type if provided.
                                let file_part = if let Some(mime) = mime_type {
                                    file_part.mime_str(&mime).expect("Invalid MIME type string")
                                } else {
                                    file_part
                                };
                                form.part(name.clone(), file_part)
                            }
                        };
                    }
                    builder.multipart(form)
                }
            };
        }

        return Ok(builder);
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
    pub content: String,
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

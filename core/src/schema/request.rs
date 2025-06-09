use std::collections::HashMap;

use serde::Deserialize;

use crate::schema::{
    request_body::RequestBodySchema, request_config::RequestConfigSchema,
    request_script::RequestScriptConfigSchema,
};

/// Represents a single API request definition.
#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RequestSchema {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub config: Option<RequestConfigSchema>, // Optional config block
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>, // Optional headers block
    #[serde(default)]
    pub query: Option<HashMap<String, String>>, // Optional query block, values can be complex
    #[serde(default)]
    pub body: Option<RequestBodySchema>, // Optional body block
    #[serde(default)]
    pub script: Option<RequestScriptConfigSchema>, // Optional script block
}

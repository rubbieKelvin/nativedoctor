use std::collections::HashMap;

use super::{
    calls::CallSchema, env::EnvironmentVariableSchema, request_body::RequestBodySchema,
    request_config::RequestConfigSchema,
};

use super::project::ProjectDefinationSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ProjectRootSchema {
    pub project: ProjectDefinationSchema,
    #[serde(default)]
    pub requests_dir: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, EnvironmentVariableSchema>,
    pub calls: CallSchema,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RequestRootSchema {
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
}

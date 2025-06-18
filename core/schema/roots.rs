use std::collections::HashMap;

use crate::schema::{
    calls::CallSchema, env::EnvironmentVariableSchema, request_body::RequestBodySchema,
    request_config::RequestConfigSchema,
};

use super::project::ProjectDefinationSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ProjectRootSchema {
    pub project: ProjectDefinationSchema,
    #[serde(default)]
    requests_dir: Option<String>,
    #[serde(default)]
    env: HashMap<String, EnvironmentVariableSchema>,
    calls: CallSchema,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

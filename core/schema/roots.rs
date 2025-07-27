use std::collections::HashMap;

use crate::schema::{
    env::EnvironmentVariableSchema, request_body::RequestBodySchema,
    request_config::RequestConfigSchema,
};

use super::project::ProjectDefinationSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ProjectRootSchema {
    pub project: ProjectDefinationSchema,
    pub calls: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RequestRootSchema {
    // i'll use this as the file name instead
    // pub ref_id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub config: Option<RequestConfigSchema>, // Optional config block
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>, // Optional headers block
    #[serde(default)]
    pub query: Option<Vec<(String, String)>>, // Optional query block, values can be complex
    #[serde(default)]
    pub body: Option<RequestBodySchema>, // Optional body block
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct EnvironmentRootSchema {
    // pub ref_id: String,
    pub name: String,
    pub description: String,
    pub variables: Vec<EnvironmentVariableSchema>,
}

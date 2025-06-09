use serde::Deserialize;
use std::collections::HashMap;

use crate::schema::{
    env::EnvironmentVariableSchema, project::ProjectDefinationSchema, request::RequestSchema,
};

/// Represents the entire API test file structure.
#[derive(Debug, Deserialize, Default)]
pub struct RootSchema {
    #[serde(default)] // Make imports optional
    pub imports: Vec<String>,
    #[serde(default)] // Make env optional
    pub env: HashMap<String, EnvironmentVariableSchema>,
    #[serde(default)] // Make requests optional
    pub requests: HashMap<String, RequestSchema>,
    #[serde(default)] // Make calls optional
    pub calls: HashMap<String, Vec<String>>,
    /// Project defination, just for more information
    #[serde(default)]
    pub project: Option<ProjectDefinationSchema>,
}

use serde::Deserialize;

use crate::schema::user::UserSchema;

/// Used to describe the project from a root file.
/// Might contain project configurations too
#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub struct ProjectDefinationSchema {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub authors: Vec<UserSchema>,
    /// This is the openapi file that this project will be continousely generated from if specified
    #[serde(default)]
    pub generator: Option<String>,
}

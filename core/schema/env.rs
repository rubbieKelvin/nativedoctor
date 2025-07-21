use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct EnvironmentVariableSchema {
    pub name: String,
    pub value: serde_yaml::Value,
    pub description: String,
    pub secret: bool,
}

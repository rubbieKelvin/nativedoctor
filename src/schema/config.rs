use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Enviroment {
    pub name: String,
    pub variables: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration {
    pub requests: Vec<String>,
    pub environments: Vec<Enviroment>,
}

impl BaseConfiguration {
    pub fn new() -> Self {
        return BaseConfiguration {
            requests: vec![],
            environments: vec![],
        };
    }
}

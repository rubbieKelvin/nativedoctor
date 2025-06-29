use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use nativedoctor_core::schema::{
    request_body::RequestBodySchema, request_config::RetryConfigSchema,
};

#[derive(PartialEq, Clone)]
pub struct Session {
    path: PathBuf,
    name: String,
    description: String,
    version: String,
    requests: Vec<RequestDefination>,
    calls: HashMap<String, Vec<String>>,
    env: HashMap<String, HashMap<String, String>>,
}

impl Session {
    #[allow(unused)]
    pub fn template() -> Self {
        return Session {
            path: PathBuf::new(),
            name: "Untitled".to_string(),
            description: String::new(),
            version: "0.0.1".to_string(),
            env: HashMap::from_iter([
                (
                    "dev".to_string(),
                    HashMap::from_iter([(
                        "baseurl".to_string(),
                        "http://localhost:8080".to_string(),
                    )]),
                ),
                (
                    "prod".to_string(),
                    HashMap::from_iter([(
                        "baseurl".to_string(),
                        "https://httpbin.org".to_string(),
                    )]),
                ),
            ]),
            requests: vec![RequestDefination {
                name: "hello".to_string(),
                method: "GET".to_string(),
                url: "{{baseurl}}/get".to_string(),
                ..Default::default()
            }],
            calls: HashMap::from_iter([("main".to_string(), vec!["hello".to_string()])]),
        };
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct RequestDefination {
    name: String,
    method: String,
    url: String,
    doc: String,
    headers: HashMap<String, String>,
    dependencies: Vec<String>,
    timeout: u32,
    retries: RetryConfigSchema,
    query: Vec<(String, String)>,
    body: Option<RequestBodySchema>,
    class: String,
}

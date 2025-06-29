use std::{collections::HashMap, path::PathBuf};

use nativedoctor_core::schema::{
    request_body::RequestBodySchema, request_config::RetryConfigSchema,
};

const _NANOID_ALPHA: [char; 19] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'x', 'y', 'z',
];

#[derive(PartialEq, Clone, Default)]
pub struct Session {
    pub path: PathBuf,
    pub name: String,
    pub description: String,
    pub version: String,
    pub requests: Vec<RequestDefination>,
    pub calls: HashMap<String, Vec<String>>,
    pub current_env: Option<String>,
    pub env: HashMap<String, HashMap<String, String>>,
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
                id: uuid::Uuid::new_v4(),
                name: "hello".to_string(),
                method: "GET".to_string(),
                url: "{{baseurl}}/get".to_string(),
                ..Default::default()
            }],
            calls: HashMap::from_iter([("main".to_string(), vec!["hello".to_string()])]),
            ..Default::default()
        };
    }

    pub fn get_environments(&self) -> Vec<String> {
        return self.env.keys().map(|k| k.clone()).collect();
    }

    pub fn new_empty_request(&mut self) -> uuid::Uuid {
        let id = uuid::Uuid::new_v4();
        self.requests.push(RequestDefination {
            id: id.clone(),
            name: "untitled".to_string(),
            method: "GET".to_string(),
            ..Default::default()
        });
        return id;
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct RequestDefination {
    pub id: uuid::Uuid,
    pub name: String,
    pub method: String,
    pub url: String,
    pub doc: String,
    pub headers: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub timeout: u32,
    pub retries: RetryConfigSchema,
    pub query: Vec<(String, String)>,
    pub body: Option<RequestBodySchema>,
    pub class: String,
}

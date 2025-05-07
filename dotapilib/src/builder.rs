use crate::types::HttpMethod;
use std::collections::HashMap;

#[allow(unused)]
pub struct RequestBuilder {
    name: String,
    url: String,
    method: HttpMethod,
    query_params: Vec<(String, String)>,
    headers: HashMap<String, String>,
}

impl RequestBuilder {
    pub fn new(name: String, url: String) -> Self {
        return RequestBuilder {
            name,
            url,
            method: HttpMethod::GET,
            query_params: vec![],
            headers: HashMap::new(),
        };
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct EnvVar {
    pub key: String,
    pub default: String,
    pub overrides: HashMap<String, String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct EnvBlock {
    pub variables: Vec<EnvVar>,
}

#[allow(unused)]
#[derive(Debug)]
pub enum Body {
    Json(String),
    GraphQL(String),
    Multipart {
        fields: HashMap<String, String>,
        files: HashMap<String, String>,
    },
    Raw(String),
    None,
}

#[allow(unused)]
#[derive(Debug)]
pub struct RequestSchema {
    pub name: String,
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query: Vec<(String, String)>,
    pub body: Body,
    pub expects: Option<String>,
    pub extracts: Vec<(String, String)>, // (var, jsonpath)
    pub asserts: Vec<String>,
    pub depends_on: Option<String>,
    pub delay: Option<u64>,
}

use nativedoctor_core::schema::{
    request_body::{MultipartPartSchema, RequestBodySchema},
    request_config::RetryConfigSchema,
};
use serde_yaml::{Mapping, Value};
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

#[derive(PartialEq, Clone, Default)]
pub(crate) struct Session {
    pub path: PathBuf,
    pub name: String,
    pub description: String,
    pub version: String,
    pub requests: Vec<RequestDefination>,
    pub calls: HashMap<String, Vec<String>>,
    pub current_env: Option<String>,
    pub default_environment: EnvironmentDefination,
    pub custom_environments: Vec<EnvironmentDefination>,
}

impl Session {
    pub fn get_environments(&self) -> Vec<EnvironmentDefination> {
        let mut envs = vec![self.default_environment.clone()];
        envs.extend(self.custom_environments.iter().map(|e| e.clone()));
        return envs;
    }

    #[allow(unused)]
    pub fn template() -> Self {
        let mut post_mapping = Mapping::new();
        post_mapping.insert(
            Value::String("user".to_string()),
            Value::String("John Doe".to_string()),
        );
        post_mapping.insert(
            Value::String("action".to_string()),
            Value::String("create".to_string()),
        );
        post_mapping.insert(Value::String("isActive".to_string()), Value::Bool(true));

        let mut put_mapping = Mapping::new();
        put_mapping.insert(
            Value::String("id".to_string()),
            Value::String("{{item_id}}".to_string()),
        );
        put_mapping.insert(
            Value::String("status".to_string()),
            Value::String("updated".to_string()),
        );

        return Session {
            path: PathBuf::new(),
            name: "httpbin.org API Demo".to_string(),
            description: "A session template demonstrating various API calls and body types using httpbin.org.".to_string(),
            version: "1.1.0".to_string(),
            default_environment: EnvironmentDefination { id: Uuid::new_v4(), ref_id: nanoid::nanoid!(), name: "Default".to_string(), path: None, variables: HashMap::from_iter([
                ("baseurl".to_string(), VariableValue::new("http://localhost:8080")),
                ("auth_token".to_string(), VariableValue::new("local-dev-token")),
                ("item_id".to_string(), VariableValue::new("abcde")),
            ]) },
            custom_environments: vec![EnvironmentDefination{id: Uuid::new_v4(), ref_id: nanoid::nanoid!(), name: "Production".to_string(), path: None, variables: HashMap::from_iter([
                ("baseurl".to_string(), VariableValue::new("https://httpbin.org")),
                ("auth_token".to_string(), VariableValue::new("your-secret-token-here")),
                ("item_id".to_string(), VariableValue::new("12345")),
            ])}],
            requests: vec![
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Get request with params".to_string(),
                    method: "GET".to_string(),
                    url: "{{baseurl}}/get".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    doc: "Sends a GET request with query parameters.".to_string(),
                    query: vec![
                        ("source".to_string(), "nativedoctor".to_string()),
                        ("page".to_string(), "1".to_string())
                        ],
                    ..Default::default()
                },
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Post json data".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    method: "POST".to_string(),
                    url: "{{baseurl}}/post".to_string(),
                    doc: "Sends a POST request with a JSON body.".to_string(),
                    headers: HashMap::from_iter([
                        ("Content-Type".to_string(), "application/json".to_string()),
                    ]),
                    body: Some(RequestBodySchema::Json {
                        content: Value::Mapping(post_mapping)
                    }),
                    ..Default::default()
                },
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Put request".to_string(),
                    method: "PUT".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    url: "{{baseurl}}/put".to_string(),
                    doc: "Sends a PUT request, similar to POST.".to_string(),
                    headers: HashMap::from_iter([
                        ("Content-Type".to_string(), "application/json".to_string()),
                    ]),
                    body: Some(RequestBodySchema::Json {
                        content: Value::Mapping(put_mapping)
                    }),
                    ..Default::default()
                },
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Upload file multipart".to_string(),
                    method: "POST".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    url: "{{baseurl}}/post".to_string(),
                    doc: "Sends a multipart/form-data request with fields and a file.".to_string(),
                    body: Some(RequestBodySchema::Multipart {
                        parts: vec![
                            MultipartPartSchema::Field { name: "description".to_string(), value: "A sample file upload".to_string() },
                            MultipartPartSchema::Field { name: "user_id".to_string(), value: "{{item_id}}".to_string() },
                            MultipartPartSchema::File { name: "upload_file".to_string(), path: "/path/to/your/file.txt".to_string(), mime_type: Some("text/plain".to_string()) }
                        ]
                    }),
                    ..Default::default()
                },
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Delete something".to_string(),
                    method: "DELETE".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    url: "{{baseurl}}/delete".to_string(),
                    doc: "Sends a DELETE request.".to_string(),
                    ..Default::default()
                },
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    name: "Check bearer".to_string(),
                    method: "GET".to_string(),
                    ref_id: nanoid::nanoid!(8),
                    url: "{{baseurl}}/bearer".to_string(),
                    doc: "Tests bearer token authentication.".to_string(),
                    headers: HashMap::from_iter([
                        ("Authorization".to_string(), "Bearer {{auth_token}}".to_string()),
                    ]),
                    ..Default::default()
                },
            ],
            calls: HashMap::from_iter([
                ("full_http_methods_test".to_string(), vec![
                    "get_request_with_params".to_string(),
                    "post_json_data".to_string(),
                    "put_request".to_string(),
                    "delete_request".to_string()
                    ]),
                ("auth_and_upload_flow".to_string(), vec![
                    "check_bearer_token".to_string(),
                    "upload_file_multipart".to_string()
                    ]),
                ]),
            ..Default::default()
        };
    }

    pub fn new_empty_request(&mut self) -> RequestDefination {
        let id = uuid::Uuid::new_v4();
        let defination = RequestDefination {
            id: id.clone(),
            name: "untitled".to_string(),
            method: "GET".to_string(),
            ..Default::default()
        };

        self.requests.push(defination.clone());
        return defination;
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct RequestDefination {
    /// this id is used as keys for the ui.
    /// this is not always the same every time this struct is created
    pub id: uuid::Uuid,
    /// ref id is used to identify this object.
    /// it's persisted and will be saved with the request in file
    pub ref_id: String,
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
    pub path: Option<PathBuf>,
}

impl RequestDefination {
    pub fn slug(&self) -> String {
        return slug::slugify(self.name.clone());
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct EnvironmentDefination {
    pub id: uuid::Uuid,
    pub ref_id: String,
    pub name: String,
    pub path: Option<PathBuf>,
    pub variables: HashMap<String, VariableValue>,
}

impl Into<String> for EnvironmentDefination {
    fn into(self) -> String {
        return self.name.clone();
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct VariableValue {
    pub value: String,
    pub sensitive: bool,
    pub description: String,
}

impl VariableValue {
    pub fn new(value: &str) -> Self {
        return VariableValue {
            value: value.to_string(),
            sensitive: false,
            description: String::new(),
        };
    }
}

impl Into<String> for VariableValue {
    fn into(self) -> String {
        return self.value;
    }
}

impl Into<VariableValue> for String {
    fn into(self) -> VariableValue {
        return VariableValue {
            value: self,
            sensitive: false,
            description: String::new(),
        };
    }
}

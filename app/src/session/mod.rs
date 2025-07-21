// this file contains the struct that model project data internally (a project in session)

use components_lib::prelude::CellValue;
use nativedoctor_core::schema::{
    env::EnvironmentVariableSchema,
    request_body::RequestBodySchema,
    request_config::{RequestConfigSchema, RetryConfigSchema},
    roots::{EnvironmentRootSchema, RequestRootSchema},
};
use serde_yaml::{Mapping, Value};
use std::{collections::HashMap, path::PathBuf};

use crate::views::project::EnvTableColumn;

mod casting;
mod fs;

#[derive(PartialEq, Clone, Default)]
pub(crate) struct Session {
    pub path: Option<PathBuf>,
    pub name: String,
    pub description: String,
    pub version: String,
    pub requests: Vec<RequestDefination>,
    pub calls: HashMap<String, Vec<String>>,
    pub current_env: Option<String>,
    pub environments: Vec<EnvironmentDefination>,
}

impl Session {
    pub fn get_environments(&self) -> Vec<EnvironmentDefination> {
        let envs = self
            .environments
            .iter()
            .map(|e| e.clone())
            .collect::<Vec<EnvironmentDefination>>();
        return envs;
    }

    #[allow(unused)]
    pub fn template() -> Self {
        // post Request body
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

        // Dev enviromnent
        let dev_enviromnent = EnvironmentDefination::new("Dev").with_variables(vec![
            (
                "baseurl".to_string(),
                VariableValue::new("http://localhost:8080"),
            ),
            (
                "auth_token".to_string(),
                VariableValue::new("secret-token-here").as_secret(),
            ),
            ("item_id".to_string(), VariableValue::new("00000")),
        ]);

        // Production environment
        let production_env = EnvironmentDefination::new("Production").with_variables(vec![
            (
                "baseurl".to_string(),
                VariableValue::new("https://httpbin.org"),
            ),
            (
                "auth_token".to_string(),
                VariableValue::new("your-secret-token-here").as_secret(),
            ),
            ("item_id".to_string(), VariableValue::new("12345")),
        ]);

        let get_request_ref_id = nanoid::nanoid!();
        let post_request_ref_id = nanoid::nanoid!();

        return Session {
            path: None,
            name: "Untitled Project".to_string(),
            description: "A session template demonstrating various API calls and body types using httpbin.org.".to_string(),
            version: "0.0.1".to_string(),
            environments: vec![
                dev_enviromnent,
                production_env
            ],
            requests: vec![
                RequestDefination {
                    id: uuid::Uuid::new_v4(),
                    ref_id: get_request_ref_id.clone(),
                    name: "Get request with params".to_string(),
                    method: "GET".to_string(),
                    url: "{{baseurl}}/get".to_string(),
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
                    ref_id: post_request_ref_id.clone(),
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
            ],
            calls: HashMap::from_iter([
                (
                    "main".to_string(),
                    vec![
                        get_request_ref_id,
                        post_request_ref_id
                    ]
                ),
            ]),
            ..Default::default()
        };
    }

    #[deprecated]
    pub fn new_empty_request(&mut self) -> RequestDefination {
        // TODO: If we can use this without adding to the session
        // let's remove this abstraction
        // we should create a ::new function
        // the same applies to other project resources, (calls, envs)
        let id = uuid::Uuid::new_v4();
        let defination = RequestDefination {
            id: id.clone(),
            name: "untitled".to_string(),
            method: "GET".to_string(),
            ..Default::default()
        };

        // maybe do not add this to the request list yet...
        // self.requests.push(defination.clone());
        return defination;
    }

    // TODO: finish
    pub fn close(&mut self) {
        tracing::warn!("Closing project. not implemented.");
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
    // TODO: i'm not sure if i need this (REMOVE)
    // If i need this, add a doc string...
    #[allow(unused)]
    pub fn slug(&self) -> String {
        return slug::slugify(self.name.clone());
    }

    // Convert tp request to save in schema
    pub fn to_request_schema(&self) -> RequestRootSchema {
        return RequestRootSchema {
            ref_id: self.ref_id.clone(),
            method: self.method.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            doc: self.doc.clone(),
            headers: if self.headers.is_empty() {
                None
            } else {
                Some(self.headers.clone())
            },
            query: if self.query.is_empty() {
                None
            } else {
                Some(self.query.clone())
            },
            config: Some(RequestConfigSchema {
                require: self.dependencies.clone(),
                timeout: Some(self.timeout.clone()),
                class: Some(self.class.clone()),
                retries: self.retries.clone(),
                ..Default::default()
            }),
            body: self.body.clone(),
        };
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

impl EnvironmentDefination {
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        let name = name.as_ref();

        return EnvironmentDefination {
            id: uuid::Uuid::new_v4(),
            ref_id: nanoid::nanoid!(),
            name: name.to_string(),
            path: None,
            variables: HashMap::new(),
        };
    }

    pub fn with_variables(mut self, variables: Vec<(String, VariableValue)>) -> Self {
        for (key, variable) in variables {
            self.variables.insert(key, variable);
        }
        return self;
    }

    pub fn into_table_data(&self) -> Vec<HashMap<String, CellValue>> {
        return self
            .variables
            .iter()
            .map(|(name, data)| {
                let mut map = HashMap::new();

                map.insert(
                    EnvTableColumn::Name.to_string(),
                    CellValue::Text(name.clone()),
                );
                map.insert(
                    EnvTableColumn::Sensitive.to_string(),
                    CellValue::Boolean(data.sensitive),
                );
                map.insert(
                    EnvTableColumn::InitialValue.to_string(),
                    CellValue::Text(data.initial.clone()),
                );
                map.insert(
                    EnvTableColumn::Value.to_string(),
                    CellValue::Text(data.value.clone()),
                );

                return map;
            })
            .collect();
    }

    // TODO: Check if i still need this after implementing the env table
    #[allow(unused)]
    pub fn make_variables_from_table_data(
        data: Vec<HashMap<String, CellValue>>,
    ) -> HashMap<String, VariableValue> {
        let mut result = HashMap::new();

        for row in data {
            let name = row
                .get(&EnvTableColumn::Name.to_string())
                .map(|cell| cell.to_string().unwrap_or_default())
                .unwrap_or_default();
            let name = name.trim().to_string();

            if name.len() == 0 {
                continue;
            }

            let mut value = VariableValue::new("");
            value.sensitive = row
                .get(&EnvTableColumn::Sensitive.to_string())
                .map(|cell| cell.to_boolean().unwrap_or_default())
                .unwrap_or_default();

            value.initial = row
                .get(&EnvTableColumn::InitialValue.to_string())
                .map(|cell| cell.to_string().unwrap_or_default())
                .unwrap_or_default();

            value.value = row
                .get(&EnvTableColumn::Value.to_string())
                .map(|cell| cell.to_string().unwrap_or_default())
                .unwrap_or_default();

            result.insert(name, value);
        }
        return result;
    }

    // Convert this struct to the schema we can save in file
    pub fn to_environment_schema(&self) -> EnvironmentRootSchema {
        return EnvironmentRootSchema {
            ref_id: self.ref_id.clone(),
            name: self.name.clone(),
            description: "".to_string(),
            variables: self
                .variables
                .iter()
                .map(|(key, value)| EnvironmentVariableSchema {
                    name: key.clone(),
                    value: serde_yaml::Value::String(value.value.clone()),
                    description: value.description.clone(),
                    secret: value.sensitive.clone(),
                })
                .collect::<Vec<EnvironmentVariableSchema>>(),
        };
    }
}

impl Into<String> for EnvironmentDefination {
    fn into(self) -> String {
        return self.name.clone();
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct VariableValue {
    pub value: String,
    pub initial: String,
    pub sensitive: bool,
    pub description: String,
}

impl VariableValue {
    pub fn new(value: &str) -> Self {
        return VariableValue {
            value: value.to_string(),
            initial: value.to_string(),
            sensitive: false,
            description: String::new(),
        };
    }

    pub fn as_secret(mut self) -> Self {
        self.sensitive = true;
        return self;
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
            value: self.clone(),
            initial: self.clone(),
            sensitive: false,
            description: String::new(),
        };
    }
}

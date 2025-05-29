use dioxus::prelude::{use_context_provider, Signal};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub description: String,
    pub enabled: bool,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    CONNECT,
    TRACE,
}

impl HttpMethod {
    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
            HttpMethod::TRACE => "TRACE".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestItem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SequenceItem {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub name: String,
    pub description: String,
}

pub fn provide_context() {
    use_context_provider::<Signal<Vec<Environment>>>(|| {
        Signal::new({
            vec![
                Environment {
                    name: "Development".to_string(),
                    description: "The development environment".to_string(),
                },
                Environment {
                    name: "Staging".to_string(),
                    description: "The staging environment".to_string(),
                },
                Environment {
                    name: "Production".to_string(),
                    description: "The production environment".to_string(),
                },
            ]
        })
    });

    use_context_provider::<Signal<Vec<EnvironmentVariable>>>(|| {
        Signal::new({
            vec![
                EnvironmentVariable {
                    name: "API_URL".to_string(),
                    value: "https://api.example.com".to_string(),
                    description: "The URL of the API".to_string(),
                    enabled: true,
                },
                EnvironmentVariable {
                    name: "API_KEY".to_string(),
                    value: "1234567890".to_string(),
                    description: "The API key".to_string(),
                    enabled: true,
                },
                EnvironmentVariable {
                    name: "API_SECRET".to_string(),
                    value: "1234567890".to_string(),
                    description: "The API secret".to_string(),
                    enabled: true,
                },
                EnvironmentVariable {
                    name: "API_TOKEN".to_string(),
                    value: "1234567890".to_string(),
                    description: "The API token".to_string(),
                    enabled: true,
                },
                EnvironmentVariable {
                    name: "API_TOKEN".to_string(),
                    value: "1234567890".to_string(),
                    description: "The API token".to_string(),
                    enabled: true,
                },
                EnvironmentVariable {
                    name: "API_TOKEN".to_string(),
                    value: "1234567890".to_string(),
                    description: "The API token".to_string(),
                    enabled: true,
                },
            ]
        })
    });
    use_context_provider::<Signal<Vec<RequestItem>>>(|| {
        Signal::new(vec![
            RequestItem {
                id: "1".to_string(),
                name: "Request 1".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::GET,
            },
            RequestItem {
                id: "2".to_string(),
                name: "Request 2".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::POST,
            },
            RequestItem {
                id: "3".to_string(),
                name: "Request 3".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::PUT,
            },
            RequestItem {
                id: "4".to_string(),
                name: "Request 4".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::DELETE,
            },
            RequestItem {
                id: "5".to_string(),
                name: "Request 5".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::PATCH,
            },
            RequestItem {
                id: "6".to_string(),
                name: "Request 6".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::PATCH,
            },
            RequestItem {
                id: "7".to_string(),
                name: "Request 7".to_string(),
                url: "https://api.example.com".to_string(),
                method: HttpMethod::OPTIONS,
            },
        ])
    });
    use_context_provider::<Signal<Vec<SequenceItem>>>(|| Signal::new(vec![]));
}

use dioxus::prelude::{use_context_provider, Signal};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestItem {
    pub name: String,
    pub url: String,
    pub method: String,
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
    use_context_provider::<Signal<Vec<RequestItem>>>(|| Signal::new(vec![
        RequestItem {
            name: "Request 1".to_string(),
            url: "https://api.example.com".to_string(),
            method: "GET".to_string(),
        },
        RequestItem {
            name: "Request 2".to_string(),
            url: "https://api.example.com".to_string(),
            method: "POST".to_string(),
        },
        RequestItem {
            name: "Request 3".to_string(),
            url: "https://api.example.com".to_string(),
            method: "PUT".to_string(),
        },
        RequestItem {
            name: "Request 4".to_string(),
            url: "https://api.example.com".to_string(),
            method: "DELETE".to_string(),
        },
        RequestItem {
            name: "Request 5".to_string(),
            url: "https://api.example.com".to_string(),
            method: "PATCH".to_string(),
        },
        RequestItem {
            name: "Request 6".to_string(),
            url: "https://api.example.com".to_string(),
            method: "PATCH".to_string(),
        },
        RequestItem {
            name: "Request 7".to_string(),
            url: "https://api.example.com".to_string(),
            method: "PATCH".to_string(),
        },
    ]));
    use_context_provider::<Signal<Vec<SequenceItem>>>(|| Signal::new(vec![]));
}

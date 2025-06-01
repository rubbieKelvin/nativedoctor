use crate::appdata::tabs::{TabItem, TabItemManager, TabType};
use dioxus::prelude::{use_context_provider, Signal};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub description: String,
    pub enabled: bool,
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
    use_context_provider::<Signal<TabItemManager>>(|| {
        Signal::new(TabItemManager {
            current_tab: Some(0),
            tabs: vec![TabItem::new(
                "Welcome".to_string(),
                TabType::WelcomePage,
                None,
            )],
        })
    });

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

    use_context_provider::<Signal<Vec<SequenceItem>>>(|| Signal::new(vec![]));
}

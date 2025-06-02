use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::Signal,
};
use uuid::Uuid;

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

    pub fn all() -> Vec<HttpMethod> {
        return vec![
            HttpMethod::GET,
            HttpMethod::POST,
            HttpMethod::PUT,
            HttpMethod::DELETE,
            HttpMethod::PATCH,
            HttpMethod::OPTIONS,
            HttpMethod::HEAD,
            HttpMethod::CONNECT,
            HttpMethod::TRACE,
        ];
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestItem {
    pub id: String,
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
}

impl RequestItem {
    fn new() -> Self {
        return RequestItem {
            id: Uuid::new_v4().to_string(),
            name: "New request".to_string(),
            url: String::new(),
            method: HttpMethod::GET,
        };
    }
    pub fn copy_from(&mut self, other: RequestItem) {
        self.id = other.id;
        self.method = other.method;
        self.url = other.url;
        self.name = other.name;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestManager {
    pub current: Option<RequestItem>,
    pub items: Vec<RequestItem>,
}

impl RequestManager {
    pub fn provide() {
        use_context_provider::<Signal<RequestManager>>(|| {
            Signal::new(RequestManager {
                current: None,
                items: vec![],
            })
        });
    }

    pub fn inject() -> Signal<RequestManager> {
        return use_context::<Signal<RequestManager>>();
    }

    pub fn insert_new(&mut self) {
        self.items.push(RequestItem::new());
    }

    pub fn delete(&mut self, id: String) {
        self.items.retain(|item| item.id != id);
    }

    pub fn update<F>(&mut self, id: String, update_fn: F)
    where
        F: FnOnce(&mut RequestItem),
    {
        let index = self.items.iter().position(|item| item.id == id);
        if index.is_none() {
            return;
        }

        let request = self.items.get_mut(index.unwrap());
        if let Some(request) = request {
            update_fn(request);
        }
    }
}

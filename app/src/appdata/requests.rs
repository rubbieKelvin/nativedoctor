use dioxus::{hooks::{use_context, use_context_provider}, signals::{Readable, Signal}};

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

// impl RequestItem {
//     pub fn copy_from(&mut self, other: RequestItem) {
//         self.id = other.id;
//         self.method = other.method;
//         self.url = other.url;
//         self.name = other.name;
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct RequestManager {
    pub current: Option<RequestItem>,
    pub items: Vec<RequestItem>
}

impl RequestManager {
    pub fn provide(){
        use_context_provider::<Signal<RequestManager>>(|| {
            Signal::new(RequestManager { current: None, items: vec![] })
        });
    }

    pub fn inject() -> Signal<RequestManager>{
        return use_context::<Signal<RequestManager>>();
    }

    pub fn get_request_items() -> Vec<RequestItem> {
        let signal = RequestManager::inject();
        let manager = signal();
        return manager.items;
    }
}
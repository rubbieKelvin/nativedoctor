use nd_core::models::requestroot::RequestRootModel;
use ratatui::widgets::TableState;
use reqwest::{
    StatusCode, Url, Version,
    header::{CONTENT_TYPE, HeaderMap},
};

use crate::app::request::enums::{RequestTab, ResponseTab};

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub running: bool,
    pub output_pane_visible: bool,
    pub is_making_request: bool,
    pub request_tab: RequestTab,
    pub response_tab: ResponseTab,
    pub initial_requestmodel: Option<RequestRootModel>,
    pub requestmodel: RequestRootModel,
    pub response: Option<reqwest::Result<Response>>,
    pub widget_states: WidgetStates,
}

#[derive(Debug, Default)]
pub struct WidgetStates {
    pub response_header_table_state: TableState,
    // pub response_header_scroll_state: u16,
}

impl SingleRequestAppState {
    /// Check if there's any change made to the loaded model
    pub fn has_model_changed(&mut self) -> bool {
        match &self.initial_requestmodel {
            Some(model) => *model == self.requestmodel,
            None => true,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Response {
    pub url: Url,
    pub version: Version,
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub string_body: Option<String>,
}

impl Response {
    pub fn from_reqwest_response(response: reqwest::blocking::Response) -> Self {
        let url = response.url().clone();
        let headers = response.headers().clone();
        let version = response.version();
        let status = response.status();

        let mut string_body: Option<String> = None;
        let content_type = headers.get(CONTENT_TYPE);

        // check if we can represent this as a string in the terminal
        // for now we can only show text
        if status.as_u16() != 204
            && let Some(content_type) = content_type
            && let Ok(content_type) = content_type.to_str()
        {
            // TODO: Handle this better
            if content_type.starts_with("text/")
                || content_type.contains("json")
                || content_type.contains("xml")
                || content_type.contains("javascript")
            {
                if let Ok(body) = response.text() {
                    string_body = Some(body.clone());
                }
            } else {
                println!("Response body is likely binary: {}", content_type);
            }
        }

        return Self {
            url,
            headers,
            version,
            status,
            string_body,
        };
    }
}

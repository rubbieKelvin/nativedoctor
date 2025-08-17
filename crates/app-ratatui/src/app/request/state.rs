use nd_core::models::requestroot::RequestRootModel;
use reqwest::blocking::Response;

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

use nd_core::models::requestroot::RequestRootModel;
use reqwest::blocking::Response;
use strum::IntoEnumIterator;

use crate::app::request::enums::{RequestTab, ResponseTab};

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub running: bool,
    pub output_pane_visible: bool,
    pub is_making_request: bool,
    pub request_tab_index: usize,
    pub response_tab_index: usize,
    pub initial_requestmodel: Option<RequestRootModel>,
    pub requestmodel: RequestRootModel,
    pub response: Option<reqwest::Result<Response>>,
}

impl SingleRequestAppState {
    pub fn get_request_tab(&mut self) -> RequestTab {
        let arr = RequestTab::iter().collect::<Vec<RequestTab>>();
        return arr[self.request_tab_index].clone();
    }

    pub fn get_response_tab(&mut self) -> ResponseTab {
        let arr = ResponseTab::iter().collect::<Vec<ResponseTab>>();
        return arr[self.response_tab_index].clone();
    }

    /// Check if there's any change made to the loaded model
    pub fn has_model_changed(&mut self) -> bool {
        match &self.initial_requestmodel {
            Some(model) => *model == self.requestmodel,
            None => true,
        }
    }
}

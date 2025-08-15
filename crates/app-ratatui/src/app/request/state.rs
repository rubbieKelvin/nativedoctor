use strum::IntoEnumIterator;

use crate::app::request::enums::{RequestMethod, RequestTab, ResponseTab};

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub running: bool,
    pub output_pane_visible: bool,
    pub method_index: usize,
    pub request_tab_index: usize,
    pub response_tab_index: usize,
    pub initial_model_state: Option<models::requestroot::RequestRootModel>,
    pub model_state: models::requestroot::RequestRootModel,
}

impl SingleRequestAppState {
    pub fn get_method(&mut self) -> RequestMethod {
        let arr = RequestMethod::iter().collect::<Vec<RequestMethod>>();
        return arr[self.method_index].clone();
    }

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
        match &self.initial_model_state {
            Some(model) => *model == self.model_state,
            None => true,
        }
    }
}

use strum::IntoEnumIterator;

use crate::{
    app::request::enums::{InputState, RequestMethod, RequestTab, ResponseTab},
    widgets::input::TextInputState,
};

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub url: TextInputState,
    pub name: TextInputState,
    pub running: bool,
    pub output_pane_visible: bool,
    pub input_state: InputState,
    pub method_index: usize,
    pub request_tab_index: usize,
    pub response_tab_index: usize,
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
}

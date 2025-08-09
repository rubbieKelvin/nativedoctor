use ratatui::{
    Frame,
    text::Line,
    widgets::{StatefulWidget, Widget},
};

use crate::app::request::{SingleRequestApp, SingleRequestAppState};

impl SingleRequestApp {
    pub fn draw(&mut self, frame: &mut Frame, state: &mut SingleRequestAppState) {
        frame.render_stateful_widget(self, frame.area(), state);
    }
}

impl StatefulWidget for &mut SingleRequestApp {
    type State = SingleRequestAppState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _state: &mut Self::State,
    ) {
        Line::from("Request").render(area, buf);
    }
}

use ratatui::{
    Frame,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::request::{InputState, SingleRequestApp, SingleRequestAppState},
    commands::ActiveInput,
};

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
        state: &mut Self::State,
    ) {
        let style = Style {
            // empty state
            fg: Some(if state.url.is_empty() {
                Color::Gray
            } else {
                Color::White
            }),
            ..Default::default()
        };

        // active state
        let style = if let InputState::Editing {
            which: ActiveInput::Url,
        } = state.input_state
        {
            style.underlined()
        } else {
            style
        };

        if state.url.is_empty() {
            Paragraph::new("Ex: https://httpbin.org/get")
                .style(style)
                .render(area, buf);
        } else {
            Paragraph::new(state.url.clone()).render(area, buf);
        }
    }
}

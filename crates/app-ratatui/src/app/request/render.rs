use crate::{
    app::request::{InputState, SingleRequestApp, SingleRequestAppState},
    commands::ActiveInput,
    widgets::input::TextInput,
};
use ratatui::{Frame, widgets::StatefulWidget};

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
        TextInput::default()
            .set_placeholder("Ex: https://httpbin.org/get")
            .set_active(matches!(
                state.input_state,
                InputState::Editing {
                    which: ActiveInput::Url
                }
            ))
            .render(area, buf, &mut state.url);
    }
}

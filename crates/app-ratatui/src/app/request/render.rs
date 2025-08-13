use crate::{
    app::request::{InputState, RequestTab, SingleRequestApp, SingleRequestAppState},
    commands::ActiveInput,
    style::KEY_SHORTCUT_FG_HINT,
    widgets::input::TextInput,
};

use ratatui::{
    Frame,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, StatefulWidget, Widget},
};

impl SingleRequestApp {
    pub fn draw(&mut self, frame: &mut Frame, state: &mut SingleRequestAppState) {
        frame.render_stateful_widget(self, frame.area(), state);
    }

    fn make_url_input(&mut self, state: &mut SingleRequestAppState) -> TextInput {
        return TextInput::default()
            .set_placeholder("Ex: https://httpbin.org/get")
            .set_active(matches!(
                state.input_state,
                InputState::Editing {
                    which: ActiveInput::Url
                }
            ));
    }

    fn make_request_tab_line(&mut self, state: &mut SingleRequestAppState) -> Vec<Span<'static>> {
        let mut request_tab_line: Vec<Span<'static>> =
            vec![Span::from(" < ").fg(KEY_SHORTCUT_FG_HINT)];

        request_tab_line.extend(RequestTab::all().iter().enumerate().map(|(index, t)| {
            let s = Span::from(if index == RequestTab::all().len() - 1 {
                format!("{} ", t.to_string())
            } else {
                format!("{} · ", t.to_string())
            });

            if t.clone() == state.request_tab.clone() {
                s.fg(KEY_SHORTCUT_FG_HINT)
            } else {
                s
            }
        }));
        request_tab_line.push(Span::from("> ").fg(KEY_SHORTCUT_FG_HINT));
        return request_tab_line;
    }

    fn draw_request_input_block(&mut self, state: &mut SingleRequestAppState) -> Block<'static> {
        let mut url_input = self.make_url_input(state);
        let request_tab_line = self.make_request_tab_line(state);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            // File name
            .title(" untitled.ndr ")
            // Http method
            .title(Line::from(vec![
                " m".fg(KEY_SHORTCUT_FG_HINT),
                "/".into(),
                state.method.span(),
                " ".into(),
            ]))
            // Url input
            .title(url_input.line_from(
                &mut state.url,
                vec![Span::from(" u").yellow(), Span::from("rl: ")],
            ))
            // Request tabs
            .title(Line::from(request_tab_line).right_aligned());

        // Show the editing state in the block's bottom line
        block = if let InputState::Editing { which } = &state.input_state {
            block.title_bottom(
                Line::from(format!(" editing {} ⮐ ", which.to_string().to_lowercase()))
                    .fg(KEY_SHORTCUT_FG_HINT),
            )
        } else {
            block
                .title_bottom(
                    Line::from(vec![" Send ".into(), " ⮐ ".fg(KEY_SHORTCUT_FG_HINT)])
                        .right_aligned(),
                )
                .title_bottom(Line::from(vec![
                    " q".fg(KEY_SHORTCUT_FG_HINT),
                    "uit ".into(),
                ]))
        };

        return block;
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
        let request_input_block = self.draw_request_input_block(state);
        request_input_block.render(area, buf);
    }
}

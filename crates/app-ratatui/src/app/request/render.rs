use crate::{
    app::request::{
        InputState, RequestTab, ResponseTab, SingleRequestApp, SingleRequestAppState,
        enums::ActiveInput,
    },
    style::KEY_SHORTCUT_FG_HINT,
    widgets::input::TextInput,
};

use nd_core::constants::APPLICATION_NAME;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use strum::IntoEnumIterator;

impl SingleRequestApp {
    pub fn draw(&mut self, frame: &mut Frame, state: &mut SingleRequestAppState) {
        frame.render_stateful_widget(self, frame.area(), state);
    }

    fn make_request_tab_line(&mut self, state: &mut SingleRequestAppState) -> Vec<Span<'static>> {
        let mut request_tab_line: Vec<Span<'static>> =
            vec![Span::from(" < ").fg(KEY_SHORTCUT_FG_HINT)];

        request_tab_line.extend(RequestTab::iter().enumerate().map(|(index, t)| {
            let s = Span::from(if index == RequestTab::iter().count() - 1 {
                format!("{} ", t.to_string())
            } else {
                format!("{} · ", t.to_string())
            });

            if t == state.request_tab {
                s.fg(KEY_SHORTCUT_FG_HINT)
            } else {
                s
            }
        }));
        request_tab_line.push(Span::from("> ").fg(KEY_SHORTCUT_FG_HINT));
        return request_tab_line;
    }

    fn make_response_tab_line(&mut self, state: &mut SingleRequestAppState) -> Vec<Span<'static>> {
        let mut request_tab_line: Vec<Span<'static>> =
            vec![Span::from("<b ").fg(KEY_SHORTCUT_FG_HINT)];

        request_tab_line.extend(ResponseTab::iter().enumerate().map(|(index, t)| {
            let s = Span::from(if index == RequestTab::iter().count() - 1 {
                format!("{} ", t.to_string())
            } else {
                format!("{} · ", t.to_string())
            });

            if t == state.response_tab {
                s.fg(KEY_SHORTCUT_FG_HINT)
            } else {
                s
            }
        }));
        request_tab_line.push(Span::from("n>").fg(KEY_SHORTCUT_FG_HINT));
        return request_tab_line;
    }

    fn render_title_block(&mut self, state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let mut n_input = TextInput::default()
            .set_placeholder("title")
            .set_active(matches!(
                self.input_state,
                InputState::Editing {
                    which: ActiveInput::RequestTitle
                }
            ));

        let style = n_input.get_input_style(&mut self.title_input_state);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![" t".fg(KEY_SHORTCUT_FG_HINT), "itle ".into()])
            .title(
                // Model change status
                Line::from_iter([
                    if state.has_model_changed() { "* " } else { "" },
                    // Filename
                    "untitled.ndr".into(),
                ])
                .centered(),
            );

        block = if let InputState::Editing {
            which: ActiveInput::RequestTitle,
        } = &self.input_state
        {
            block.title_bottom(" editing ⮐ ")
        } else {
            block
        };

        let para = Paragraph::new(n_input.text(&mut self.title_input_state)).style(style);
        return para.block(block);
    }

    fn render_url_input_block(&mut self, _state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let mut u_input = TextInput::default()
            .set_placeholder("https://httpbin.org/get")
            .set_active(matches!(
                self.input_state,
                InputState::Editing {
                    which: ActiveInput::RequestUrl
                }
            ));

        let style = u_input.get_input_style(&mut self.url_input_state);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![Span::from(" u").yellow(), Span::from("rl ")]);

        block = if let InputState::Editing { which } = self.input_state.clone() {
            match which {
                ActiveInput::RequestUrl => block.title_bottom(" editing ⮐ "),
                _ => block,
            }
        } else {
            block.title(
                Line::from(vec![" send ".into(), "⮐ ".fg(KEY_SHORTCUT_FG_HINT)]).right_aligned(),
            )
        };

        let para = Paragraph::new(u_input.text(&mut self.url_input_state)).style(style);
        return para.block(block);
    }

    fn render_methods_block(&mut self, state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![" m".fg(KEY_SHORTCUT_FG_HINT), "ethod ".into()]);
        let para = Paragraph::new(state.requestmodel.method.to_string());
        return para.block(block);
    }

    fn render_request_input_area(&mut self, state: &mut SingleRequestAppState) -> Block<'static> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(self.make_request_tab_line(state));
        return block;
    }

    fn render_response_output_area(&mut self, state: &mut SingleRequestAppState) -> Block<'static> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![" ¹".fg(KEY_SHORTCUT_FG_HINT), "output ".into()])
            .title(Line::from(self.make_response_tab_line(state)).right_aligned())
            .title_bottom(vec![" q".fg(KEY_SHORTCUT_FG_HINT), "uit ".into()])
            .title_bottom(
                Line::from(
                    if let InputState::Editing { which } = self.input_state.clone() {
                        format!(" input mode: {} ", which.to_string().to_lowercase())
                    } else {
                        String::from(" command mode ")
                    },
                )
                .blue(),
            )
            .title_bottom(Line::from(APPLICATION_NAME.to_lowercase()).right_aligned());

        return block;
    }

    fn render_inner_response_output_tab_area(
        &mut self,
        state: &mut SingleRequestAppState,
    ) -> impl Widget {
        return match state.response_tab {
            ResponseTab::Headers => self.render_response_header_tab(state),
            ResponseTab::Body => self.render_response_body_tab(state),
            ResponseTab::Log => self.render_response_log_tab(state),
        };
    }

    fn render_response_header_tab(&mut self, state: &mut SingleRequestAppState) -> &'static str {
        return "Response headers";
    }

    fn render_response_body_tab(&mut self, state: &mut SingleRequestAppState) -> &'static str {
        return "Body";
    }

    fn render_response_log_tab(&mut self, state: &mut SingleRequestAppState) -> &'static str {
        return "Log";
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
        let [title_area, url_area, view_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .areas(area);

        // title area split
        let [methods_area, title_area] =
            Layout::horizontal([Constraint::Max(10), Constraint::Min(20)]).areas(title_area);

        let [request_input_area, response_output_area] = if state.output_pane_visible {
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
        } else {
            Layout::vertical([Constraint::Min(3), Constraint::Max(3)])
        }
        .areas(view_area);

        self.render_methods_block(state).render(methods_area, buf);
        self.render_url_input_block(state).render(url_area, buf);
        self.render_title_block(state).render(title_area, buf);
        self.render_request_input_area(state)
            .render(request_input_area, buf);

        let response_output_block = self.render_response_output_area(state);
        let response_output_inner = response_output_block.inner(response_output_area);
        response_output_block.render(response_output_area, buf);
        self.render_inner_response_output_tab_area(state)
            .render(response_output_inner, buf);
    }
}

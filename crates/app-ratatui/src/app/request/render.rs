use crate::{
    app::request::{InputState, RequestTab, ResponseTab, SingleRequestApp, SingleRequestAppState},
    commands::{self, ActiveInput},
    style::KEY_SHORTCUT_FG_HINT,
    widgets::input::TextInput,
};

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

    fn make_response_tab_line(&mut self, state: &mut SingleRequestAppState) -> Vec<Span<'static>> {
        let mut request_tab_line: Vec<Span<'static>> =
            vec![Span::from(" b ").fg(KEY_SHORTCUT_FG_HINT)];

        request_tab_line.extend(ResponseTab::iter().enumerate().map(|(index, t)| {
            let s = Span::from(if index == RequestTab::all().len() - 1 {
                format!("{} ", t.to_string())
            } else {
                format!("{} · ", t.to_string())
            });

            if t.clone() == state.response_tab.clone() {
                s.fg(KEY_SHORTCUT_FG_HINT)
            } else {
                s
            }
        }));
        request_tab_line.push(Span::from("n ").fg(KEY_SHORTCUT_FG_HINT));
        return request_tab_line;
    }

    fn render_title_block(&mut self, state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let mut n_input = TextInput::default()
            .set_placeholder("title")
            .set_active(matches!(
                state.input_state,
                InputState::Editing {
                    which: ActiveInput::RequestTitle
                }
            ));

        let style = n_input.get_input_style(&mut state.name);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![" t".fg(KEY_SHORTCUT_FG_HINT), "itle ".into()])
            .title(Line::from("untitled.ndr").centered());

        block = if let InputState::Editing {
            which: commands::ActiveInput::RequestTitle,
        } = &state.input_state
        {
            block.title_bottom(" editing ⮐ ")
        } else {
            block
        };

        let para = Paragraph::new(n_input.text(&mut state.name)).style(style);
        return para.block(block);
    }

    fn render_url_input_block(&mut self, state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let mut u_input = TextInput::default()
            .set_placeholder("Ex: https://httpbin.org/get")
            .set_active(matches!(
                state.input_state,
                InputState::Editing {
                    which: ActiveInput::RequestUrl
                }
            ));

        let style = u_input.get_input_style(&mut state.url);

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![Span::from(" u").yellow(), Span::from("rl ")]);

        block = if let InputState::Editing {
            which: commands::ActiveInput::RequestUrl,
        } = state.input_state.clone()
        {
            block.title_bottom(" editing ⮐ ")
        } else {
            block
        };

        let para = Paragraph::new(u_input.text(&mut state.url)).style(style);
        return para.block(block);
    }

    fn render_methods_block(&mut self, state: &mut SingleRequestAppState) -> Paragraph<'static> {
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(vec![" m".fg(KEY_SHORTCUT_FG_HINT), "ethod ".into()]);
        let para = Paragraph::new(state.method.span());
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
            .title(vec![" o".fg(KEY_SHORTCUT_FG_HINT), "utput ".into()])
            .title(Line::from(self.make_response_tab_line(state)).right_aligned())
            .title_bottom(vec![" q".fg(KEY_SHORTCUT_FG_HINT), "uit ".into()])
            .title_bottom(
                Line::from(
                    if let InputState::Editing { which } = state.input_state.clone() {
                        format!(" input mode: {} ", which.to_string().to_lowercase())
                    } else {
                        String::from(" command mode ")
                    },
                )
                .blue(),
            )
            .title_bottom(Line::from("nativedoctor v0.0.1 ").right_aligned());
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
        self.render_response_output_area(state)
            .render(response_output_area, buf);
    }
}

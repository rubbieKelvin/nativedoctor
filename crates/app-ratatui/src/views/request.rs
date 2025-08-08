use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode},
    layout::Rect,
    text::Line,
    widgets::Widget,
};

use crate::{commands, state, views::View};

pub struct RequestPage;

impl RequestPage {
    pub fn new() -> Self {
        return RequestPage;
    }
}

impl View for RequestPage {
    fn render(&self, area: Rect, buf: &mut Buffer, _state: &state::ApplicationState) {
        Line::from("Rubbie the one").render(area, buf);
    }

    fn handle_key_event(
        &mut self,
        key: event::KeyEvent,
        state: &mut state::ApplicationState,
    ) -> io::Result<Option<commands::Command>> {
        match key.code {
            KeyCode::Char('q') => {
                state.running = false;
            }
            _ => {}
        };
        return Ok(None);
    }
}

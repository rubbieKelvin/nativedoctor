use ratatui::{
    layout::Position,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{StatefulWidget, Widget},
};

use crate::style::KEY_SHORTCUT_FG_HINT;

#[derive(Default, Debug)]
pub struct TextInputState {
    pub index: u16,
    pub value: String,
    pub position: Position,
}

impl Into<String> for TextInputState {
    fn into(self) -> String {
        return self.value.clone();
    }
}

impl Into<TextInputState> for String {
    fn into(self) -> TextInputState {
        return TextInputState {
            value: self,
            ..Default::default()
        };
    }
}

impl Into<TextInputState> for &'static str {
    fn into(self) -> TextInputState {
        return TextInputState {
            value: self.to_string(),
            ..Default::default()
        };
    }
}

impl TextInputState {
    pub fn paste<S: AsRef<str>>(&mut self, string: S) {
        let string = string.as_ref();
        self.value.insert_str(self.index as usize, string);
        self.index += string.len() as u16;
    }

    pub fn push(&mut self, ch: char) {

        self.value.insert(self.index as usize, ch);
        self.index += 1;
    }

    pub fn pop(&mut self) {
        if self.index == 0 {
            return;
        }

        self.index -= 1;
        self.value.remove(self.index as usize);
    }
}

#[derive(Default)]
pub struct TextInput {
    label: Option<Span<'static>>,
    active: bool,
    placeholder: String,
}

impl TextInput {
    pub fn set_placeholder<S: AsRef<str>>(mut self, placeholder: S) -> Self {
        self.placeholder = placeholder.as_ref().to_string();
        return self;
    }

    pub fn set_active(mut self, active: bool) -> Self {
        self.active = active;
        return self;
    }

    pub fn set_label(mut self, label: Span<'static>) -> Self {
        self.label = Some(label);
        return self;
    }

    pub fn get_input_style(&mut self, state: &mut TextInputState) -> Style {
        let style = Style {
            // empty state
            fg: Some(if state.value.is_empty() {
                Color::Gray
            } else {
                Color::White
            }),
            ..Default::default()
        };

        // let style = if self.active { style.bold() } else { style };

        return style;
    }

    pub fn text(&mut self, state: &mut TextInputState) -> String {
        return if state.value.is_empty() {
            self.placeholder.clone()
        } else {
            state.value.clone()
        };
    }

    pub fn line_from<'a>(
        &mut self,
        state: &mut TextInputState,
        mut start: Vec<Span<'a>>,
    ) -> Line<'a> {
        let style = self.get_input_style(state);
        let text = self.text(state);

        if let Some(label) = &self.label {
            start.push(label.clone());
        }

        start.push(Span::from(text).style(style));
        if self.active {
            start.push(Span::from(" ⮐").fg(KEY_SHORTCUT_FG_HINT));
        }
        start.push(Span::from(" "));

        return Line::from(start);
    }

    pub fn line<'a>(&mut self, state: &mut TextInputState) -> Line<'a> {
        return self.line_from(state, vec![]);
    }
}

impl StatefulWidget for &mut TextInput {
    type State = TextInputState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let style = self.get_input_style(state);
        Span::from(self.text(state)).style(style).render(area, buf);
    }
}

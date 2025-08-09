use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Default, Debug)]
pub struct TextInputState {
    pub value: String,
}

impl Into<String> for TextInputState {
    fn into(self) -> String {
        return self.value.clone();
    }
}

impl Into<TextInputState> for String {
    fn into(self) -> TextInputState {
        return TextInputState { value: self };
    }
}

impl Into<TextInputState> for &'static str {
    fn into(self) -> TextInputState {
        return TextInputState {
            value: self.to_string(),
        };
    }
}

#[derive(Default)]
pub struct TextInput {
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

    fn get_input_style(&mut self, state: &mut TextInputState) -> Style {
        let style = Style {
            // empty state
            fg: Some(if state.value.is_empty() {
                Color::Gray
            } else {
                Color::White
            }),
            ..Default::default()
        };

        let style = if self.active {
            style.underlined()
        } else {
            style
        };

        return style;
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

        let paragraph = if state.value.is_empty() {
            Paragraph::new("Ex: https://httpbin.org/get")
        } else {
            Paragraph::new(state.value.clone())
        };

        paragraph.style(style).render(area, buf);
    }
}

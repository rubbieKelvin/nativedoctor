use gpui::*;
use gpui_component::{
    input::{Input, InputState},
    Theme,
};

pub fn render(_theme: &Theme, input_state: &Entity<InputState>) -> impl IntoElement {
    return div()
        .size_full()
        .child(Input::new(input_state).appearance(false).size_full().px_0());
}

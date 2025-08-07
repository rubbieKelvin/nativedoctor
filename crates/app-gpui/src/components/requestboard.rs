use gpui::{ParentElement, Styled, Window, div, rgb};

pub fn render<T>(_window: &mut Window, _cx: &mut gpui::Context<T>) -> impl gpui::IntoElement {
    return div().bg(rgb(0xfff)).size_full().child("Hello");
}

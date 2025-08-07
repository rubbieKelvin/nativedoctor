use gpui::{div, IntoElement};

use crate::views::View;

pub struct RequestView;

impl View for RequestView {
    fn render<T>(&mut self, cx: &gpui::Context<T>) -> impl IntoElement {
        return div();
    }
}

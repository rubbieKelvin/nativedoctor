use gpui::{Context, IntoElement, Window};

use crate::components::requestboard;

pub fn render<T>(window: &mut Window, cx: &mut Context<T>) -> impl IntoElement {
    return requestboard::render(window, cx);
}

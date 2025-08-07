use gpui::{Context, IntoElement, Window};

mod request;

pub enum ViewType {
    Request,
}

pub struct ViewManager {
    current: ViewType,
}

impl ViewManager {
    pub fn new() -> Self {
        return ViewManager {
            current: ViewType::Request,
        };
    }

    pub fn render<T>(&mut self, window: &mut Window, cx: &mut Context<T>) -> impl IntoElement {
        return match self.current {
            ViewType::Request => request::render(window, cx),
        };
    }
}

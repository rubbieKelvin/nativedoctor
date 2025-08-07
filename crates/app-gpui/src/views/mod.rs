use std::collections::HashMap;

use gpui::{Context, IntoElement, Styled, div, rgb};

mod request;

pub enum ViewType {
    Request,
}

pub trait View {
    fn render<T>(&mut self, cx: &Context<T>) -> impl IntoElement;
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

    pub fn render<T>(&mut self, cx: &mut Context<T>) -> impl IntoElement {
        return div().bg(rgb(0xffffff)).size_full();
    }
}

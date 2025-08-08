use gpui::{Context, Entity, IntoElement, Window};

use crate::{app::NativeDoctor, runtime::RuntimeData, states::ApplicationState};

mod request;

#[derive(Clone)]
pub enum ViewType {
    Request,
}

#[derive(Clone)]
pub struct ViewManager {
    current: ViewType,
}

impl ViewManager {
    pub fn new() -> Self {
        return ViewManager {
            current: ViewType::Request,
        };
    }

    pub fn render(
        &self,
        window: &mut Window,
        cx: &mut Context<NativeDoctor>,
        state: Entity<ApplicationState>,
        runtime: Entity<RuntimeData>,
    ) -> impl IntoElement {
        return match self.current {
            ViewType::Request => request::render(window, cx, state, runtime),
        };
    }
}

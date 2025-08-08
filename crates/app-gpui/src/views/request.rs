use gpui::{Context, Entity, IntoElement, Window};

use crate::{
    app::NativeDoctor, components::requestboard, runtime::RuntimeData, states::ApplicationState,
};

pub fn render(
    window: &mut Window,
    cx: &mut Context<NativeDoctor>,
    state: Entity<ApplicationState>,
    runtime: Entity<RuntimeData>,
) -> impl IntoElement {
    return requestboard::render(window, cx, state, runtime);
}

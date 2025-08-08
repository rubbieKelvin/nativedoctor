use gpui::{Context, Entity, IntoElement, ParentElement, Styled, Window, div, px};
use nd_core::constants;

use crate::{app::NativeDoctor, runtime::RuntimeData, states::ApplicationState};

pub fn render(
    _window: &mut Window,
    cx: &mut Context<NativeDoctor>,
    state: Entity<ApplicationState>,
    _runtime: Entity<RuntimeData>,
) -> impl IntoElement {
    let state_ref = state.read(cx);
    let theme = &state_ref.theme;

    return div()
        .bg(theme.background)
        .border_b_1()
        .border_color(theme.stroke)
        .h(px(32.0))
        .px(px(80.0))
        .flex()
        .items_center()
        .text_color(theme.text)
        .text_size(theme.font_size_normal)
        .child(constants::APPLICATION_NAME);
}

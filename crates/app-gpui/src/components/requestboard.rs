use gpui::{Entity, ParentElement, Styled, Window, div, rgb};
use gpui_component::{resizable::h_resizable, v_flex};

use crate::{app::NativeDoctor, runtime::RuntimeData, states::ApplicationState};

pub fn render(
    _window: &mut Window,
    _cx: &mut gpui::Context<NativeDoctor>,
    _state: Entity<ApplicationState>,
    _runtime: Entity<RuntimeData>,
) -> impl gpui::IntoElement {
    return div().flex_grow().child(v_flex());
}

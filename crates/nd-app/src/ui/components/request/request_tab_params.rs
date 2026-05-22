use gpui::*;
use gpui_component::{StyledExt, Theme};

use crate::ui::components::kvinput;

pub fn render(_theme: &Theme, state: &Entity<kvinput::KvInputState>) -> impl IntoElement {
    return div().child(kvinput::KvInput::new(state));
}

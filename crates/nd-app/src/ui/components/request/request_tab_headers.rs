use gpui::*;
use gpui_component::Theme;

use crate::ui::components::kvinput;

pub fn render(_theme: &Theme, state: &Entity<kvinput::KvInputState>) -> impl IntoElement {
    return div().child(kvinput::KvInput::new(state));
}

use gpui::*;
use gpui_component::Theme;

use super::RequestPanel;

pub fn render(theme: &Theme, cx: &mut Context<RequestPanel>) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(auth_type_view(theme))
        .child(auth_footer(theme));
}

fn auth_type_view(theme: &Theme) -> impl IntoElement {
    return div()
        .p_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .flex_1();
}

fn auth_footer(theme: &Theme) -> impl IntoElement {
    // show select to slect auth type. with a a circulat i icon that holds a tool tip explaining the currently selected auth type.
    return div().px_2().child("child");
}

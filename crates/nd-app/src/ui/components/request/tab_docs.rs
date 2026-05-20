use gpui::*;
use gpui_component::{StyledExt, Theme};

pub fn render(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .p_4()
        .gap_2()
        .child(
            div()
                .text_xs()
                .font_semibold()
                .text_color(theme.muted_foreground)
                .child("Description"),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .p_3()
                .rounded(px(4.))
                .bg(theme.muted.opacity(0.05))
                .border(px(1.))
                .border_color(theme.border)
                .text_sm()
                .text_color(theme.muted_foreground)
                .child("Add a description for this request..."),
        );
}

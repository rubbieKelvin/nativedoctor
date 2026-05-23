use gpui::*;
use gpui_component::{StyledExt, Theme};

pub fn render(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .px_4()
        .py_2()
        .gap_1()
        .child(response_header_row("content-type", "application/json", theme))
        .child(response_header_row("server", "nginx/1.21", theme))
        .child(response_header_row("cache-control", "no-cache", theme));
}

fn response_header_row(key: &str, value: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_2()
        .text_xs()
        .child(
            div()
                .text_color(theme.foreground)
                .font_semibold()
                .child(SharedString::from(key)),
        )
        .child(
            div()
                .text_color(theme.muted_foreground)
                .child(SharedString::from(value)),
        );
}

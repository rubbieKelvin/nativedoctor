use gpui::*;
use gpui_component::{StyledExt, Theme};

pub fn render(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .gap_1()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .px_4()
                .pt_4()
                .pb_2()
                .child(
                    div()
                        .text_xs()
                        .font_semibold()
                        .text_color(theme.muted_foreground)
                        .child("Query Parameters"),
                ),
        )
        .child(params_table_header(theme))
        .child(param_row("search", "hello", true, theme))
        .child(param_row("limit", "10", true, theme))
        .child(param_row("offset", "0", false, theme))
        .child(param_row("", "", false, theme))
        .child(div().flex_1().min_h_0());
}

fn params_table_header(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .px_4()
        .py_1()
        .gap_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .bg(theme.muted.opacity(0.04))
        .child(
            div()
                .flex_1()
                .text_xs()
                .font_semibold()
                .text_color(theme.muted_foreground)
                .child("Key"),
        )
        .child(
            div()
                .flex_1()
                .text_xs()
                .font_semibold()
                .text_color(theme.muted_foreground)
                .child("Value"),
        )
        .child(
            div()
                .flex_1()
                .text_xs()
                .font_semibold()
                .text_color(theme.muted_foreground)
                .child("Description"),
        )
        .child(div().w(px(24.)));
}

fn param_row(key: &str, value: &str, filled: bool, theme: &Theme) -> impl IntoElement {
    let fg = if filled {
        theme.foreground
    } else {
        theme.muted_foreground
    };

    return div()
        .flex()
        .flex_row()
        .items_center()
        .px_4()
        .py_1()
        .gap_2()
        .border_b(px(1.))
        .border_color(theme.border.opacity(0.5))
        .child(
            div()
                .flex_1()
                .text_sm()
                .text_color(fg)
                .child(SharedString::from(key)),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .text_color(fg)
                .child(SharedString::from(value)),
        )
        .child(
            div()
                .flex_1()
                .text_sm()
                .text_color(theme.muted_foreground)
                .child(""),
        )
        .child(div().w(px(24.)));
}

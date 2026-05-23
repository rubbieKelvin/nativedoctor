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
        .gap_2()
        .child(timeline_row("DNS Lookup", "2 ms", theme))
        .child(timeline_row("TCP Handshake", "15 ms", theme))
        .child(timeline_row("TLS Handshake", "28 ms", theme))
        .child(timeline_row("Server Processing", "65 ms", theme))
        .child(timeline_row("Content Transfer", "5 ms", theme))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .pt_2()
                .border_t(px(1.))
                .border_color(theme.border.opacity(0.3))
                .child(timeline_label("Total", theme))
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.foreground)
                        .child("115 ms"),
                ),
        );
}

fn timeline_row(label: &str, value: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_2()
        .text_xs()
        .child(timeline_label(label, theme))
        .child(
            div()
                .text_color(theme.muted_foreground)
                .child(SharedString::from(value)),
        );
}

fn timeline_label(label: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .w(px(140.))
        .text_color(theme.foreground)
        .font_semibold()
        .child(SharedString::from(label));
}

use gpui::*;
use gpui_component::Theme;

pub fn render(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .px_4()
        .py_2()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .text_xs()
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("2024-01-15 10:30:45"),
                )
                .child(
                    div()
                        .text_color(theme.foreground)
                        .child("[INFO]"),
                )
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("Request sent to https://api.example.com/users"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .text_xs()
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("2024-01-15 10:30:46"),
                )
                .child(
                    div()
                        .text_color(theme.green)
                        .child("[OK]"),
                )
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .child("Response received with status 200"),
                ),
        );
}

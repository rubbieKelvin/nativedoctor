use gpui::*;
use gpui_component::Theme;

pub fn render(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .child(
            div()
                .flex_1()
                .min_h_0()
                .px_4()
                .py_2()
                .text_sm()
                .text_color(theme.muted_foreground)
                .child("{\n  \"id\": 1,\n  \"name\": \"John Doe\",\n  \"email\": \"john@example.com\"\n}"),
        );
}

use gpui::*;
use gpui_component::{input, StyledExt, Theme};

use super::RequestPanel;

pub fn render(
    body_type: usize,
    body_text_state: &Entity<input::InputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .gap_1()
        .child(body_type_bar(body_type, theme, cx));
    // .child(body_editor(body_type, body_text_state, theme));
}

fn body_type_bar(
    body_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_0p5()
        .px_4()
        .py_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .child(body_type_option("None", 0, body_type, theme, cx))
        .child(body_type_option("JSON", 1, body_type, theme, cx))
        .child(body_type_option("Form Data", 2, body_type, theme, cx))
        .child(body_type_option("Raw", 3, body_type, theme, cx));
}

fn body_type_option(
    label: &str,
    index: usize,
    current: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    let is_active = index == current;
    let fg = if is_active {
        theme.foreground
    } else {
        theme.muted_foreground
    };
    let bg = if is_active {
        theme.blue.opacity(0.12)
    } else {
        hsla(0., 0., 0., 0.)
    };
    let label = label.to_string();
    let id = format!("body-{}", label.to_lowercase().replace(' ', "-"));

    return div()
        .id(ElementId::Name(id.into()))
        .px_2()
        .py_1()
        .rounded(px(4.))
        .bg(bg)
        .text_color(fg)
        .text_xs()
        .font_semibold()
        .cursor_pointer()
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            this.body_type = index;
        }))
        .child(label);
}

fn body_editor(
    body_type: usize,
    body_text_state: &Entity<input::InputState>,
    theme: &Theme,
) -> impl IntoElement {
    if body_type == 0 {
        return div()
            .flex_1()
            .min_h_0()
            .flex()
            .items_center()
            .justify_center()
            .text_sm()
            .text_color(theme.muted_foreground)
            .child("This request does not have a body");
    }

    return div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .p_4()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .pb_2()
                .child(match body_type {
                    1 => "JSON",
                    2 => "Form Data",
                    3 => "Raw",
                    _ => "",
                }),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .rounded(px(4.))
                .border(px(1.))
                .border_color(theme.border)
                .child(input::Input::new(body_text_state)),
        );
}

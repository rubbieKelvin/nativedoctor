use gpui::*;
use gpui_component::{
    input::{Input, InputState},
    Icon, IconName, StyledExt, Theme,
};

use super::RequestPanel;
use crate::ui::components::kvinput;

pub fn render(
    body_type: usize,
    body_text_state: &Entity<InputState>,
    form_data_state: &Entity<kvinput::KvInputState>,
    url_encoded_state: &Entity<kvinput::KvInputState>,
    raw_sub_type: usize,
    _binary_path: &SharedString,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(body_type_bar(body_type, theme, cx))
        .child(body_form(
            body_type,
            body_text_state,
            form_data_state,
            url_encoded_state,
            raw_sub_type,
            theme,
            cx,
        ));
}

// ---------------------------------------------------------------------------
// Body type bar — pill selector
// ---------------------------------------------------------------------------

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
        .child(body_type_option("Form Data", 1, body_type, theme, cx))
        .child(body_type_option("URL Encoded", 2, body_type, theme, cx))
        .child(body_type_option("Raw", 3, body_type, theme, cx))
        .child(body_type_option("Binary", 4, body_type, theme, cx));
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

// ---------------------------------------------------------------------------
// Body form — dispatches to correct editor based on body_type
// ---------------------------------------------------------------------------

fn body_form(
    body_type: usize,
    body_text_state: &Entity<InputState>,
    form_data_state: &Entity<kvinput::KvInputState>,
    url_encoded_state: &Entity<kvinput::KvInputState>,
    raw_sub_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    match body_type {
        0 => body_none_empty_state(theme).into_any_element(),
        1 => body_form_data_form(form_data_state).into_any_element(),
        2 => body_url_encoded_form(url_encoded_state).into_any_element(),
        3 => body_raw_form(raw_sub_type, body_text_state, theme, cx).into_any_element(),
        4 => body_binary_empty_state(theme).into_any_element(),
        _ => div().into_any_element(),
    }
}

// ---------------------------------------------------------------------------
// None — empty state
// ---------------------------------------------------------------------------

fn body_none_empty_state(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .size_full()
        .gap_3()
        .child(
            Icon::new(IconName::CircleX)
                .size(px(32.))
                .text_color(theme.muted_foreground.opacity(0.4)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.muted_foreground)
                        .child("No Body"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground.opacity(0.7))
                        .child("This request does not have a body."),
                ),
        );
}

// ---------------------------------------------------------------------------
// Form Data — key-value editor
// ---------------------------------------------------------------------------

fn body_form_data_form(state: &Entity<kvinput::KvInputState>) -> impl IntoElement {
    return kvinput::KvInput::new(state);
}

// ---------------------------------------------------------------------------
// URL Encoded — key-value editor
// ---------------------------------------------------------------------------

fn body_url_encoded_form(state: &Entity<kvinput::KvInputState>) -> impl IntoElement {
    return kvinput::KvInput::new(state);
}

// ---------------------------------------------------------------------------
// Raw — sub-type bar + code editor
// ---------------------------------------------------------------------------

fn body_raw_form(
    raw_sub_type: usize,
    body_text_state: &Entity<InputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(body_raw_sub_type_bar(raw_sub_type, theme, cx))
        .child(body_raw_editor(raw_sub_type, body_text_state, theme));
}

fn body_raw_sub_type_bar(
    raw_sub_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_0p5()
        .px_4()
        .py_1()
        .border_b(px(1.))
        .border_color(theme.border.opacity(0.5))
        .child(raw_sub_type_option("JSON", 0, raw_sub_type, theme, cx))
        .child(raw_sub_type_option("HTML", 1, raw_sub_type, theme, cx))
        .child(raw_sub_type_option("XML", 2, raw_sub_type, theme, cx))
        .child(raw_sub_type_option("Plain", 3, raw_sub_type, theme, cx));
}

fn raw_sub_type_option(
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
    let id = format!("raw-body-{}", label.to_lowercase());

    return div()
        .id(ElementId::Name(id.into()))
        .px_2()
        .py_0p5()
        .rounded(px(4.))
        .bg(bg)
        .text_color(fg)
        .text_xs()
        .cursor_pointer()
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            this.raw_sub_type = index;
        }))
        .child(label);
}

fn body_raw_editor(
    _raw_sub_type: usize,
    body_text_state: &Entity<InputState>,
    _theme: &Theme,
) -> impl IntoElement {
    return div()
        .flex_1()
        .min_h_0()
        .child(Input::new(body_text_state).appearance(false).size_full().px_0());
}

// ---------------------------------------------------------------------------
// Binary — placeholder empty state
// ---------------------------------------------------------------------------

fn body_binary_empty_state(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .size_full()
        .gap_3()
        .child(
            Icon::new(IconName::File)
                .size(px(32.))
                .text_color(theme.muted_foreground.opacity(0.4)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_semibold()
                        .text_color(theme.muted_foreground)
                        .child("Binary File"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground.opacity(0.7))
                        .child("Select a file to attach as the request body."),
                ),
        );
}

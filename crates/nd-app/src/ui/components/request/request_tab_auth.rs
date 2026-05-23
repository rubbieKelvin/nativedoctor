use gpui::*;
use gpui_component::{
    input::{Input, InputState},
    Icon, IconName, StyledExt, Theme,
};

use super::RequestPanel;

const AUTH_DESCRIPTIONS: &[&str] = &[
    "No authentication is sent with this request.",
    "Sends an Authorization: Bearer <token> header.",
    "Sends an Authorization: Basic <base64-credentials> header.",
    "Sends a custom header or query parameter with the API key value.",
];

pub fn render(
    auth_type: usize,
    bearer_token_state: &Entity<InputState>,
    basic_auth_username_state: &Entity<InputState>,
    basic_auth_password_state: &Entity<InputState>,
    api_key_name_state: &Entity<InputState>,
    api_key_value_state: &Entity<InputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(auth_type_bar(auth_type, theme, cx))
        .child(auth_form(
            auth_type,
            bearer_token_state,
            basic_auth_username_state,
            basic_auth_password_state,
            api_key_name_state,
            api_key_value_state,
            theme,
        ))
        .child(auth_footer(auth_type, theme));
}

fn auth_type_bar(
    auth_type: usize,
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
        .child(auth_type_option("No Auth", 0, auth_type, theme, cx))
        .child(auth_type_option("Bearer Token", 1, auth_type, theme, cx))
        .child(auth_type_option("Basic Auth", 2, auth_type, theme, cx))
        .child(auth_type_option("API Key", 3, auth_type, theme, cx));
}

fn auth_type_option(
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
    let id = format!("auth-{}", label.to_lowercase().replace(' ', "-"));

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
            this.auth_type = index;
        }))
        .child(label);
}

fn auth_form(
    auth_type: usize,
    bearer_token_state: &Entity<InputState>,
    basic_auth_username_state: &Entity<InputState>,
    basic_auth_password_state: &Entity<InputState>,
    api_key_name_state: &Entity<InputState>,
    api_key_value_state: &Entity<InputState>,
    theme: &Theme,
) -> impl IntoElement {
    return div().p_2().flex_1().child(match auth_type {
        0 => auth_no_auth_empty_state(theme).into_any_element(),
        1 => auth_bearer_token_form(bearer_token_state, theme).into_any_element(),
        2 => auth_basic_auth_form(basic_auth_username_state, basic_auth_password_state, theme)
            .into_any_element(),
        3 => auth_api_key_form(api_key_name_state, api_key_value_state, theme).into_any_element(),
        _ => div().into_any_element(),
    });
}

fn auth_no_auth_empty_state(theme: &Theme) -> impl IntoElement {
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
                        .child("No Authentication"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground.opacity(0.7))
                        .child("This request does not require authentication."),
                ),
        );
}

fn auth_bearer_token_form(token_state: &Entity<InputState>, theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(auth_field_label("Token", theme))
        .child(Input::new(token_state).w_full());
}

fn auth_basic_auth_form(
    username_state: &Entity<InputState>,
    password_state: &Entity<InputState>,
    theme: &Theme,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(auth_field_label("Username", theme))
                .child(Input::new(username_state).w_full()),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(auth_field_label("Password", theme))
                .child(Input::new(password_state).w_full()),
        );
}

fn auth_api_key_form(
    key_name_state: &Entity<InputState>,
    key_value_state: &Entity<InputState>,
    theme: &Theme,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(auth_field_label("Key name", theme))
                .child(Input::new(key_name_state).w_full()),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(auth_field_label("Key value", theme))
                .child(Input::new(key_value_state).w_full()),
        );
}

fn auth_field_label(label: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .text_xs()
        .font_semibold()
        .text_color(theme.muted_foreground)
        .child(label.to_string());
}

fn auth_footer(auth_type: usize, theme: &Theme) -> impl IntoElement {
    let description = AUTH_DESCRIPTIONS.get(auth_type).unwrap_or(&"").to_string();

    return div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px_2()
        .py_1()
        .gap_2()
        .border_t(px(1.))
        .border_color(theme.border)
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground.opacity(0.7))
                .child(description),
        );
}

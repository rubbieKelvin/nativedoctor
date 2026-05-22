use gpui::*;
use gpui_component::{
    input::{Input, InputState},
    select::{Select, SelectState},
    tooltip::Tooltip,
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
    auth_type_state: &Entity<SelectState<Vec<SharedString>>>,
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
        .child(auth_type_view(
            auth_type_state,
            bearer_token_state,
            basic_auth_username_state,
            basic_auth_password_state,
            api_key_name_state,
            api_key_value_state,
            theme,
            cx,
        ))
        .child(auth_footer(auth_type_state, theme, cx));
}

fn auth_type_view(
    auth_type_state: &Entity<SelectState<Vec<SharedString>>>,
    bearer_token_state: &Entity<InputState>,
    basic_auth_username_state: &Entity<InputState>,
    basic_auth_password_state: &Entity<InputState>,
    api_key_name_state: &Entity<InputState>,
    api_key_value_state: &Entity<InputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .p_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .flex_1()
        .child(auth_form(
            auth_type_state,
            bearer_token_state,
            basic_auth_username_state,
            basic_auth_password_state,
            api_key_name_state,
            api_key_value_state,
            theme,
            cx,
        ));
}

fn auth_form(
    auth_type_state: &Entity<SelectState<Vec<SharedString>>>,
    bearer_token_state: &Entity<InputState>,
    basic_auth_username_state: &Entity<InputState>,
    basic_auth_password_state: &Entity<InputState>,
    api_key_name_state: &Entity<InputState>,
    api_key_value_state: &Entity<InputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    let selected = auth_type_state.read(cx).selected_value();

    match selected {
        Some(s) if s.as_ref() == "No Auth" => auth_no_auth_empty_state(theme).into_any_element(),
        Some(s) if s.as_ref() == "Bearer Token" => {
            auth_bearer_token_form(bearer_token_state, theme).into_any_element()
        }
        Some(s) if s.as_ref() == "Basic Auth" => {
            auth_basic_auth_form(basic_auth_username_state, basic_auth_password_state, theme)
                .into_any_element()
        }
        Some(s) if s.as_ref() == "API Key" => {
            auth_api_key_form(api_key_name_state, api_key_value_state, theme).into_any_element()
        }
        _ => div().into_any_element(),
    }
}

// ---------------------------------------------------------------------------
// No Auth — empty state
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Bearer Token
// ---------------------------------------------------------------------------

fn auth_bearer_token_form(token_state: &Entity<InputState>, theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(auth_field_label("Token", theme))
        .child(Input::new(token_state).w_full());
}

// ---------------------------------------------------------------------------
// Basic Auth
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// API Key
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn auth_field_label(label: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .text_xs()
        .font_semibold()
        .text_color(theme.muted_foreground)
        .child(label.to_string());
}

// ---------------------------------------------------------------------------
// Footer (auth type selector + info tooltip)
// ---------------------------------------------------------------------------

fn auth_footer(
    auth_type_state: &Entity<SelectState<Vec<SharedString>>>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    let current_description = match auth_type_state.read(cx).selected_value() {
        Some(s) if s.as_ref() == "No Auth" => AUTH_DESCRIPTIONS[0],
        Some(s) if s.as_ref() == "Bearer Token" => AUTH_DESCRIPTIONS[1],
        Some(s) if s.as_ref() == "Basic Auth" => AUTH_DESCRIPTIONS[2],
        Some(s) if s.as_ref() == "API Key" => AUTH_DESCRIPTIONS[3],
        _ => "",
    };
    let current_description = current_description.to_string();

    return div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .px_2()
        .py_1()
        .border_t(px(1.))
        .border_color(theme.border)
        .child(Select::new(auth_type_state).w(px(160.)))
        .child(
            div()
                .id(ElementId::Name("auth-type-info".into()))
                .size(px(18.))
                .rounded(px(999.))
                .flex()
                .items_center()
                .justify_center()
                .bg(theme.muted.opacity(0.15))
                .tooltip(move |window, cx| {
                    Tooltip::new(current_description.clone()).build(window, cx)
                })
                .child(
                    Icon::new(IconName::Info)
                        .size(px(12.))
                        .text_color(theme.muted_foreground),
                ),
        );
}

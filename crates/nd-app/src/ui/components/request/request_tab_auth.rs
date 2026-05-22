use gpui::*;
use gpui_component::{
    select::{Select, SelectState},
    tooltip::Tooltip,
    Icon, IconName, Theme,
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
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(auth_type_view(theme))
        .child(auth_footer(auth_type_state, theme, cx));
}

fn auth_type_view(theme: &Theme) -> impl IntoElement {
    return div()
        .p_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .flex_1();
}

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

use gpui::*;
use gpui_component::{tooltip::Tooltip, Icon, IconName, StyledExt, Theme};

use super::RequestPanel;

const AUTH_TYPES: &[(&str, &str)] = &[
    ("No Auth", "No authentication is sent with this request."),
    ("Bearer", "Sends an Authorization: Bearer <token> header."),
    (
        "Basic",
        "Sends an Authorization: Basic <base64-credentials> header.",
    ),
    (
        "API Key",
        "Sends a custom header or query parameter with the API key value.",
    ),
];

pub fn render(auth_type: usize, theme: &Theme, cx: &mut Context<RequestPanel>) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(auth_type_view(theme))
        .child(auth_footer(auth_type, theme, cx));
}

fn auth_type_view(theme: &Theme) -> impl IntoElement {
    return div()
        .p_2()
        .border_b(px(1.))
        .border_color(theme.border)
        .flex_1();
}

fn auth_footer(
    auth_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    let current_description = AUTH_TYPES
        .get(auth_type)
        .map(|(_, desc)| *desc)
        .unwrap_or("");

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
        .child(
            div()
                .flex()
                .flex_row()
                .gap_0p5()
                .children(AUTH_TYPES.iter().enumerate().map(|(i, (label, _))| {
                    return auth_type_option(label, i, auth_type, theme, cx);
                })),
        )
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

use gpui::*;
use gpui_component::{StyledExt, Theme};

use super::RequestPanel;

pub fn render(
    auth_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .p_4()
        .gap_4()
        .child(auth_type_header(theme))
        .child(auth_type_selector(auth_type, theme, cx))
        .child(auth_fields(auth_type, theme));
}

fn auth_type_header(theme: &Theme) -> impl IntoElement {
    return div()
        .text_xs()
        .font_semibold()
        .text_color(theme.muted_foreground)
        .child("Auth Type");
}

fn auth_type_selector(
    auth_type: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_0p5()
        .child(auth_type_option("None", 0, auth_type, theme, cx))
        .child(auth_type_option("Basic Auth", 1, auth_type, theme, cx))
        .child(auth_type_option("Bearer Token", 2, auth_type, theme, cx))
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
    let bg = if is_active {
        theme.blue.opacity(0.12)
    } else {
        hsla(0., 0., 0., 0.)
    };
    let fg = if is_active {
        theme.blue
    } else {
        theme.muted_foreground
    };
    let label = label.to_string();
    let id = format!("auth-{}", label.to_lowercase().replace(' ', "-"));

    return div()
        .id(ElementId::Name(id.into()))
        .px_3()
        .py_2()
        .bg(bg)
        .text_color(fg)
        .text_sm()
        .rounded(px(4.))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            this.auth_type = index;
        }))
        .child(label);
}

fn auth_fields(auth_type: usize, theme: &Theme) -> AnyElement {
    match auth_type {
        1 => basic_auth_fields(theme).into_any_element(),
        2 => bearer_token_field(theme).into_any_element(),
        3 => api_key_fields(theme).into_any_element(),
        _ => div().into_any_element(),
    }
}

fn basic_auth_fields(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Username"),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(4.))
                        .bg(theme.muted.opacity(0.05))
                        .border(px(1.))
                        .border_color(theme.border)
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("username"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Password"),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(4.))
                        .bg(theme.muted.opacity(0.05))
                        .border(px(1.))
                        .border_color(theme.border)
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("••••••••"),
                ),
        );
}

fn bearer_token_field(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("Token"),
        )
        .child(
            div()
                .px_3()
                .py_2()
                .rounded(px(4.))
                .bg(theme.muted.opacity(0.05))
                .border(px(1.))
                .border_color(theme.border)
                .text_sm()
                .text_color(theme.muted_foreground)
                .child("Enter bearer token..."),
        );
}

fn api_key_fields(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Key"),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(4.))
                        .bg(theme.muted.opacity(0.05))
                        .border(px(1.))
                        .border_color(theme.border)
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("X-API-Key"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Value"),
                )
                .child(
                    div()
                        .px_3()
                        .py_2()
                        .rounded(px(4.))
                        .bg(theme.muted.opacity(0.05))
                        .border(px(1.))
                        .border_color(theme.border)
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child("Enter API key..."),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("Add to"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_4()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(14.))
                                        .h(px(14.))
                                        .rounded(px(2.))
                                        .bg(theme.blue),
                                )
                                .child(div().text_sm().child("Header")),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(14.))
                                        .h(px(14.))
                                        .rounded(px(2.))
                                        .border(px(1.))
                                        .border_color(theme.border),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.muted_foreground)
                                        .child("Query param"),
                                ),
                        ),
                ),
        );
}

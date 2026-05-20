use gpui::*;
use gpui_component::{StyledExt, Theme};

use super::RequestPanel;

pub fn render(
    ssl_verify: bool,
    follow_redirects: bool,
    http_version: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .p_4()
        .gap_5()
        .child(settings_section_heading("General", theme))
        .child(http_version_selector(http_version, theme, cx))
        .child(timeout_input(theme))
        .child(settings_divider(theme))
        .child(settings_section_heading("Security", theme))
        .child(toggle_setting(
            "SSL certificate verification",
            ssl_verify,
            "Verify SSL certificates when making requests",
            theme,
            cx,
            |this| &mut this.ssl_verify,
        ))
        .child(settings_divider(theme))
        .child(settings_section_heading("Redirects", theme))
        .child(toggle_setting(
            "Follow redirects",
            follow_redirects,
            "Automatically follow HTTP redirects",
            theme,
            cx,
            |this| &mut this.follow_redirects,
        ))
        .child(max_redirects_input(theme))
        .child(div().flex_1().min_h_0());
}

fn settings_section_heading(label: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .text_xs()
        .font_semibold()
        .text_color(theme.muted_foreground)
        .child(SharedString::from(label));
}

fn settings_divider(theme: &Theme) -> impl IntoElement {
    return div().h(px(1.)).bg(theme.border.opacity(0.4));
}

fn toggle_setting(
    label: &str,
    on: bool,
    description: &str,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
    field: fn(&mut RequestPanel) -> &mut bool,
) -> impl IntoElement {
    let label = label.to_string();
    let description = description.to_string();

    return div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(div().text_sm().text_color(theme.foreground).child(label))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child(description),
                ),
        )
        .child(toggle_switch(on, theme, cx, field));
}

fn toggle_switch(
    on: bool,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
    field: fn(&mut RequestPanel) -> &mut bool,
) -> impl IntoElement {
    let bg = if on {
        theme.blue
    } else {
        theme.muted.opacity(0.3)
    };

    return div()
        .id("toggle-switch")
        .flex_shrink_0()
        .w(px(40.))
        .h(px(20.))
        .rounded(px(10.))
        .bg(bg)
        .cursor_pointer()
        .flex()
        .items_center()
        .px(px(2.))
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            let field_ref = field(this);
            *field_ref = !*field_ref;
        }))
        .child(
            div()
                .w(px(16.))
                .h(px(16.))
                .rounded(px(8.))
                .bg(white()),
        );
}

fn http_version_selector(
    http_version: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("HTTP Version"),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(version_option("HTTP/1.1", 0, http_version, theme, cx))
                .child(version_option("HTTP/2", 1, http_version, theme, cx)),
        );
}

fn version_option(
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
    let id = format!("http-version-{}", index);

    return div()
        .id(ElementId::Name(id.into()))
        .px_3()
        .py_1p5()
        .rounded(px(4.))
        .bg(bg)
        .text_color(fg)
        .text_sm()
        .cursor_pointer()
        .border(px(1.))
        .border_color(if is_active { theme.blue } else { theme.border })
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            this.http_version = index;
        }))
        .child(label);
}

fn timeout_input(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("Timeout (seconds)"),
        )
        .child(
            div()
                .w(px(128.))
                .px_3()
                .py_2()
                .rounded(px(4.))
                .bg(theme.muted.opacity(0.05))
                .border(px(1.))
                .border_color(theme.border)
                .text_sm()
                .text_color(theme.foreground)
                .child("30"),
        );
}

fn max_redirects_input(theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(theme.muted_foreground)
                .child("Max redirects"),
        )
        .child(
            div()
                .w(px(128.))
                .px_3()
                .py_2()
                .rounded(px(4.))
                .bg(theme.muted.opacity(0.05))
                .border(px(1.))
                .border_color(theme.border)
                .text_sm()
                .text_color(theme.foreground)
                .child("10"),
        );
}

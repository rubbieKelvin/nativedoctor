use gpui::*;
use gpui_component::{StyledExt, Theme};

use super::RequestPanel;
use crate::ui::components::number_input;

pub fn render(
    ssl_verify: bool,
    follow_redirects: bool,
    http_version: usize,
    timeout_state: &Entity<number_input::NumberInputState>,
    max_redirects_state: &Entity<number_input::NumberInputState>,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .child(settings_title("General", theme))
        .child(settings_row(
            "HTTP Version",
            http_version_control(http_version, theme, cx),
            theme,
        ))
        .child(settings_row(
            "Timeout",
            number_input::NumberInput::new("timeout", timeout_state, theme),
            theme,
        ))
        .child(settings_divider(theme))
        .child(settings_title("Security", theme))
        .child(toggle_row(
            "SSL certificate verification",
            ssl_verify,
            theme,
            cx,
            |this| &mut this.ssl_verify,
        ))
        .child(settings_divider(theme))
        .child(settings_title("Redirects", theme))
        .child(toggle_row(
            "Follow redirects",
            follow_redirects,
            theme,
            cx,
            |this| &mut this.follow_redirects,
        ))
        .child(settings_row(
            "Max redirects",
            number_input::NumberInput::new("max-redirects", max_redirects_state, theme),
            theme,
        ))
        .child(div().flex_1().min_h_0());
}

// ---------------------------------------------------------------------------
// Compact row primitives
// ---------------------------------------------------------------------------

fn settings_title(label: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .px_4()
        .pt_2()
        .pb_1()
        .text_xs()
        .font_bold()
        .text_color(theme.muted_foreground)
        .child(label.to_string());
}

fn settings_row(label: &str, control: impl IntoElement, theme: &Theme) -> impl IntoElement {
    return div()
        .px_4()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(32.))
        .child(
            div()
                .text_sm()
                .text_color(theme.foreground)
                .child(label.to_string()),
        )
        .child(control);
}

fn toggle_row(
    label: &str,
    on: bool,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
    field: fn(&mut RequestPanel) -> &mut bool,
) -> impl IntoElement {
    return div()
        .px_4()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(32.))
        .child(
            div()
                .text_sm()
                .text_color(theme.foreground)
                .child(label.to_string()),
        )
        .child(toggle_switch(label, on, theme, cx, field));
}

fn settings_divider(theme: &Theme) -> impl IntoElement {
    return div()
        .mx_4()
        .my_0p5()
        .h(px(1.))
        .bg(theme.border.opacity(0.3));
}

// ---------------------------------------------------------------------------
// Custom switch
// ---------------------------------------------------------------------------

fn toggle_switch(
    id: &str,
    on: bool,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
    field: fn(&mut RequestPanel) -> &mut bool,
) -> impl IntoElement {
    let id = format!("toggle-{}", id.to_lowercase().replace(' ', "-"));
    let bg = if on {
        theme.blue
    } else {
        theme.muted.opacity(0.3)
    };
    return div()
        .id(ElementId::Name(id.into()))
        .flex_shrink_0()
        .w(px(36.))
        .h(px(18.))
        .rounded(px(9.))
        .bg(bg)
        .cursor_pointer()
        .flex()
        .items_center()
        .px(px(2.))
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            let field_ref = field(this);
            *field_ref = !*field_ref;
        }))
        .child(div().w(px(14.)).h(px(14.)).rounded(px(7.)).bg(white()));
}

// ---------------------------------------------------------------------------
// HTTP Version pill selector
// ---------------------------------------------------------------------------

fn http_version_control(
    http_version: usize,
    theme: &Theme,
    cx: &mut Context<RequestPanel>,
) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_1()
        .child(version_option("HTTP/1.1", 0, http_version, theme, cx))
        .child(version_option("HTTP/2", 1, http_version, theme, cx));
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
        .px_2()
        .py_0p5()
        .rounded(px(4.))
        .bg(bg)
        .text_color(fg)
        .text_xs()
        .cursor_pointer()
        .border(px(1.))
        .border_color(if is_active { theme.blue } else { theme.border })
        .on_click(cx.listener(move |this, _event, _window, _cx| {
            this.http_version = index;
        }))
        .child(label);
}

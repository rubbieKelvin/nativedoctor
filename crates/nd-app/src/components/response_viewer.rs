//! Response viewer — shown below the request editor.

use gpui::*;

use gpui_component::ActiveTheme as _;

use crate::state::AppState;

/// Render the duplex response gutter with toolkit-aware pigments.
pub fn render_response_viewer(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let chrome = cx.theme();

    let execution = state
        .read(cx)
        .active_project
        .as_ref()
        .and_then(|snapshot| snapshot.last_execution_result.clone());

    return match execution {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(chrome.muted_foreground)
            .text_sm()
            .child("Send a request to hydrate this pane."),
        Some(record) => {
            let signal = crate::theme::status_color(record.status);

            let status_chip = if record.status == 0 {
                SharedString::from("Error")
            } else {
                SharedString::from(format!("{}", record.status))
            };

            let pacing = if record.duration_ms < 1000 {
                format!("{}ms", record.duration_ms)
            } else {
                format!("{:.2}s", record.duration_ms as f64 / 1000.0)
            };

            let payload_hint = if record.response_size < 1024 {
                format!("{}B", record.response_size)
            } else {
                format!("{:.1}KB", record.response_size as f64 / 1024.0)
            };

            let rendered_body = if record.response_body.is_empty() {
                "(empty body)".to_string()
            } else if let Ok(decoded) = serde_json::from_slice::<serde_json::Value>(&record.response_body)
            {
                serde_json::to_string_pretty(&decoded).unwrap_or_default()
            } else {
                String::from_utf8_lossy(&record.response_body).to_string()
            };

            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(chrome.background)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .px_3()
                        .py_2()
                        .border_b_1()
                        .border_color(chrome.border)
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .bg(signal)
                                .text_color(chrome.primary_foreground)
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(status_chip),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(chrome.muted_foreground)
                                .child(pacing.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(chrome.muted_foreground)
                                .child(payload_hint.clone()),
                        )
                        .child(if let Some(err) = &record.error_message {
                            div()
                                .text_xs()
                                .text_color(chrome.danger)
                                .truncate()
                                .child(err.clone())
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        }),
                )
                .child(
                    div().flex_1().p_3().child(div().text_xs().font_family(chrome.mono_font_family.clone()).child(rendered_body)),
                )
        }
    };
}

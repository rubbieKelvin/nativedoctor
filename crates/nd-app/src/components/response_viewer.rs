//! Response viewer — shown below the request editor.

use gpui::*;

use crate::state::AppState;

/// Render the response viewer.
pub fn render_response_viewer(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let result = {
        let s = state.read(cx);
        s.active_project
            .as_ref()
            .and_then(|p| p.last_execution_result.clone())
    };

    match result {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(crate::theme::text_muted())
            .text_sm()
            .child("Send a request to see the response"),
        Some(res) => {
            let status_color = crate::theme::status_color(res.status);
            let status_text = if res.status == 0 {
                "Error".to_string()
            } else {
                format!("{}", res.status)
            };
            let duration = if res.duration_ms < 1000 {
                format!("{}ms", res.duration_ms)
            } else {
                format!("{:.2}s", res.duration_ms as f64 / 1000.0)
            };
            let size = if res.response_size < 1024 {
                format!("{}B", res.response_size)
            } else {
                format!("{:.1}KB", res.response_size as f64 / 1024.0)
            };
            let body = if res.response_body.is_empty() {
                "(empty body)".to_string()
            } else if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&res.response_body)
            {
                serde_json::to_string_pretty(&val).unwrap_or_default()
            } else {
                String::from_utf8_lossy(&res.response_body).to_string()
            };

            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(crate::theme::bg_dark())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .px_3()
                        .py_2()
                        .border_b_1()
                        .border_color(crate::theme::border())
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded_md()
                                .bg(status_color)
                                .text_color(gpui::white())
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(status_text),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(crate::theme::text_secondary())
                                .child(duration),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(crate::theme::text_secondary())
                                .child(size),
                        )
                        .child(if let Some(err) = &res.error_message {
                            div()
                                .text_xs()
                                .text_color(crate::theme::status_server_error())
                                .truncate()
                                .child(err.clone())
                                .into_any_element()
                        } else {
                            div().into_any_element()
                        }),
                )
                .child(
                    div()
                        .flex_1()
                        .p_3()
                        .child(div().text_xs().font_family("Menlo, monospace").child(body)),
                )
        }
    }
}

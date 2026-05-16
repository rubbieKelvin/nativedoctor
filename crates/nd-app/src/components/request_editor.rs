//! Request editor — shown when a request is selected.

use gpui::*;

use crate::state::AppState;

/// Render the request editor.
pub fn render_request_editor(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let request = {
        let s = state.read(cx);
        let proj = s.active_project.as_ref();
        let req_id = proj.and_then(|p| p.selected_request_id.as_deref());
        match (req_id, proj) {
            (Some(id), Some(proj)) => proj.requests.iter().find(|r| r.id == id).cloned(),
            _ => None,
        }
    };

    match request {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(crate::theme::text_muted())
            .child("No request selected"),
        Some(req) => {
            let method_color = crate::theme::method_color(&req.method);
            let headers_display = match serde_json::from_str::<serde_json::Value>(&req.headers) {
                Ok(serde_json::Value::Object(map)) => map
                    .into_iter()
                    .map(|(k, v)| format!("{k}: {}", v.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
                    .join("\n"),
                _ => String::new(),
            };
            let body_display = req
                .body_content
                .clone()
                .unwrap_or_else(|| "No body".to_string());

            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(crate::theme::bg_dark())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_2()
                        .border_b_1()
                        .border_color(crate::theme::border())
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(crate::theme::bg_mid())
                                .text_color(method_color)
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(req.method.to_uppercase()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(crate::theme::bg_darkest())
                                .border_1()
                                .border_color(crate::theme::border())
                                .text_sm()
                                .child(req.url.clone()),
                        )
                        .child(
                            div()
                                .px_4()
                                .py_1()
                                .rounded_md()
                                .bg(crate::theme::green())
                                .text_color(gpui::white())
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .child("Send"),
                        )
                        .child(
                            div()
                                .px_4()
                                .py_1()
                                .rounded_md()
                                .bg(crate::theme::bg_mid())
                                .text_color(crate::theme::text_primary())
                                .border_1()
                                .border_color(crate::theme::border())
                                .text_sm()
                                .child("Save"),
                        ),
                )
                .child(
                    div().flex_1().p_3().child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(crate::theme::text_secondary())
                            .mb_2()
                            .child("Headers"),
                    ),
                )
                .child(
                    div()
                        .px_3()
                        .text_xs()
                        .mb_3()
                        .child(if headers_display.is_empty() {
                            "No headers configured".to_string()
                        } else {
                            headers_display
                        }),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(crate::theme::text_secondary())
                        .px_3()
                        .mb_2()
                        .child("Body"),
                )
                .child(
                    div()
                        .px_3()
                        .text_xs()
                        .font_family("Menlo, monospace")
                        .child(body_display.clone()),
                )
        }
    }
}

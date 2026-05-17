//! Request editor — shown when a request is selected.

use gpui::*;

use gpui_component::ActiveTheme as _;

use crate::state::AppState;

/// Render the request composer while synchronising pigments with gpui-component’s palette.
pub fn render_request_editor(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let chrome = cx.theme();

    let request = {
        let app_state = state.read(cx);
        let project_snapshot = app_state.active_project.as_ref();
        let request_key = project_snapshot.and_then(|project| project.selected_request_id.as_deref());

        match (request_key, project_snapshot) {
            (Some(id), Some(project)) => project.requests.iter().find(|needle| needle.id == id).cloned(),
            _ => None,
        }
    };

    return match request {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(chrome.muted_foreground)
            .child("No request selected"),
        Some(snapshot) => {
            let ribbon = crate::theme::method_color(&snapshot.method);
            let headers_display = match serde_json::from_str::<serde_json::Value>(&snapshot.headers) {
                Ok(serde_json::Value::Object(entries)) => entries
                    .into_iter()
                    .map(|(key, payload)| format!("{key}: {}", payload.as_str().unwrap_or("")))
                    .collect::<Vec<_>>()
                    .join("\n"),
                _ => String::new(),
            };

            let body_canvas = snapshot
                .body_content
                .clone()
                .unwrap_or_else(|| "No body configured".into());

            div()
                .size_full()
                .flex()
                .flex_col()
                .bg(chrome.background)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .py_2()
                        .border_b_1()
                        .border_color(chrome.border)
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(chrome.secondary)
                                .text_color(ribbon)
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .child(snapshot.method.to_uppercase()),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(chrome.input_background())
                                .border_1()
                                .border_color(chrome.border)
                                .text_sm()
                                .child(snapshot.url.clone()),
                        )
                        .child(
                            div()
                                .px_4()
                                .py_1()
                                .rounded_md()
                                .bg(chrome.primary)
                                .text_color(chrome.primary_foreground)
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .child("Send"),
                        )
                        .child(
                            div()
                                .px_4()
                                .py_1()
                                .rounded_md()
                                .bg(chrome.secondary)
                                .text_color(chrome.foreground)
                                .border_1()
                                .border_color(chrome.border)
                                .text_sm()
                                .child("Save"),
                        ),
                )
                .child(
                    div().flex_1().p_3().child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(chrome.muted_foreground)
                            .mb_2()
                            .child("Headers"),
                    ),
                )
                .child(
                    div().px_3().text_sm().mb_3().text_color(chrome.foreground).child(if headers_display.is_empty() {
                        "No headers configured".into()
                    } else {
                        headers_display
                    }),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(chrome.muted_foreground)
                        .px_3()
                        .mb_2()
                        .child("Body"),
                )
                .child(
                    div().px_3().text_sm().font_family(chrome.mono_font_family.clone()).child(body_canvas.clone()),
                )
        }
    };
}

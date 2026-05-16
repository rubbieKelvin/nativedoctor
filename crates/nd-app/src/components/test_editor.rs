//! Test (Rhai script) editor.

use gpui::*;

use crate::state::AppState;

/// Render the test editor.
pub fn render_test_editor(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let test = {
        let s = state.read(cx);
        let proj = s.active_project.as_ref();
        let test_id = proj.and_then(|p| p.selected_test_id.as_deref());
        match (test_id, proj) {
            (Some(id), Some(proj)) => proj.tests.iter().find(|t| t.id == id).cloned(),
            _ => None,
        }
    };

    match test {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(crate::theme::text_muted())
            .child("No test selected"),
        Some(test) => div()
            .size_full()
            .flex()
            .flex_col()
            .bg(crate::theme::bg_dark())
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(crate::theme::border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().child("🧪"))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(test.name.clone()),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_4()
                                    .py_1()
                                    .rounded_md()
                                    .bg(crate::theme::green())
                                    .text_color(gpui::white())
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("▶ Run"),
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
                    ),
            )
            .child(
                div().flex_1().p_3().child(
                    div()
                        .size_full()
                        .rounded_md()
                        .bg(crate::theme::bg_darkest())
                        .border_1()
                        .border_color(crate::theme::border())
                        .p_3()
                        .child(
                            div()
                                .text_sm()
                                .font_family("Menlo, monospace")
                                .child(test.script.clone()),
                        ),
                ),
            )
            .child(
                div()
                    .h(px(120.))
                    .border_t_1()
                    .border_color(crate::theme::border())
                    .bg(crate::theme::bg_darkest())
                    .p_3()
                    .child(
                        div()
                            .text_xs()
                            .text_color(crate::theme::text_muted())
                            .child("Output will appear here after running the test..."),
                    ),
            ),
    }
}

//! Test (Rhai script) editor.

use gpui::*;

use gpui_component::ActiveTheme as _;

use crate::state::AppState;

/// Render the Rhai workspace with gpui-component colour tokens only.
pub fn render_test_editor(
    _window: &mut Window,
    cx: &mut App,
    state: &Entity<AppState>,
) -> impl IntoElement {
    let chrome = cx.theme();

    let active = {
        let shell = state.read(cx);
        let project = shell.active_project.as_ref();
        let fingerprint = project.and_then(|layer| layer.selected_test_id.as_deref());

        match (fingerprint, project) {
            (Some(tag), Some(project)) => project.tests.iter().find(|specimen| specimen.id == tag).cloned(),
            _ => None,
        }
    };

    return match active {
        None => div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(chrome.muted_foreground)
            .child("No Rhai test selected."),
        Some(specimen) => div()
            .size_full()
            .flex()
            .flex_col()
            .bg(chrome.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(chrome.border)
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
                                    .child(specimen.name.clone()),
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
                                    .bg(chrome.primary)
                                    .text_color(chrome.primary_foreground)
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .child("▶ Run"),
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
                    ),
            )
            .child(
                div().flex_1().p_3().child(
                    div()
                        .size_full()
                        .rounded_md()
                        .bg(chrome.input_background())
                        .border_1()
                        .border_color(chrome.border)
                        .p_3()
                        .child(
                            div().text_sm().font_family(chrome.mono_font_family.clone()).child(specimen.script.clone()),
                        ),
                ),
            )
            .child(
                div()
                    .h(px(120.))
                    .border_t_1()
                    .border_color(chrome.border)
                    .bg(chrome.input_background())
                    .p_3()
                    .child(
                        div().text_sm().text_color(chrome.muted_foreground).child(
                            "Rhai evaluations will mirror nd-core runtime once bridged.",
                        ),
                    ),
            ),
    };
}

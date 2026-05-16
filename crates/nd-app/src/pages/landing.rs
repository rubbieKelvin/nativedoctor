//! Landing page — shown when no project is open.

use gpui::*;

use crate::state::AppState;

/// Render the landing page as a simple element.
pub fn render_landing(
    _window: &mut Window,
    cx: &mut App,
    state: Entity<AppState>,
) -> impl IntoElement {
    let recent = state.read(cx).recent_projects.clone();

    div()
        .size_full()
        .bg(crate::theme::bg_darkest())
        .text_color(crate::theme::text_primary())
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .mb_8()
                .child(
                    div()
                        .text_2xl()
                        .font_weight(FontWeight::BOLD)
                        .text_color(crate::theme::green())
                        .child("🏥 NativeDoctor"),
                )
                .child(
                    div()
                        .text_lg()
                        .text_color(crate::theme::text_secondary())
                        .mt_2()
                        .child("API Testing & Development Tool"),
                ),
        )
        .child(
            div()
                .flex()
                .gap_4()
                .mb_8()
                .child(
                    div()
                        .px_6()
                        .py_3()
                        .bg(crate::theme::green())
                        .text_color(gpui::white())
                        .rounded_md()
                        .font_weight(FontWeight::MEDIUM)
                        .child("Create New Project"),
                )
                .child(
                    div()
                        .px_6()
                        .py_3()
                        .bg(crate::theme::bg_mid())
                        .text_color(crate::theme::text_primary())
                        .rounded_md()
                        .border_1()
                        .border_color(crate::theme::border())
                        .font_weight(FontWeight::MEDIUM)
                        .child("Open Existing Project"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .w(px(480.))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(crate::theme::text_secondary())
                        .mb_3()
                        .child("Recent Projects"),
                )
                .child(if recent.is_empty() {
                    div()
                        .text_sm()
                        .text_color(crate::theme::text_muted())
                        .italic()
                        .child("No recent projects. Create or open one to get started.")
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .flex_col()
                        .children(recent.into_iter().map(|rp| {
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .px_4()
                                .py_2()
                                .rounded_md()
                                .bg(crate::theme::bg_mid())
                                .mb_1()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().text_sm().child("📁"))
                                        .child(div().text_sm().child(rp.name.clone())),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(crate::theme::text_muted())
                                        .child(rp.last_opened.clone()),
                                )
                                .into_any_element()
                        }))
                        .into_any_element()
                }),
        )
}

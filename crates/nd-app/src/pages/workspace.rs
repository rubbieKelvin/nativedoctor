//! Workspace page — the main view when a project is open.

use gpui::*;

use crate::state::AppState;

/// Render the workspace layout.
pub fn render_workspace(
    _window: &mut Window,
    cx: &mut App,
    state: Entity<AppState>,
) -> impl IntoElement {
    let (selected_tab, selected_request_id, selected_test_id) = {
        let s = state.read(cx);
        let proj = s.active_project.as_ref();
        let tab = proj
            .map(|p| p.selected_tab.clone())
            .unwrap_or(crate::state::SidebarTab::Requests);
        let req = proj.and_then(|p| p.selected_request_id.clone());
        let test = proj.and_then(|p| p.selected_test_id.clone());
        (tab, req, test)
    };

    div()
        .size_full()
        .flex()
        .bg(crate::theme::bg_darkest())
        .text_color(crate::theme::text_primary())
        .child(render_sidebar(cx, &state, &selected_tab))
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .child(if selected_test_id.is_some() {
                    crate::components::test_editor::render_test_editor(_window, cx, &state)
                        .into_any_element()
                } else if selected_request_id.is_some() {
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(div().flex_1().child(
                            crate::components::request_editor::render_request_editor(
                                _window, cx, &state,
                            ),
                        ))
                        .child(div().h(px(2.)).bg(crate::theme::border()))
                        .child(div().flex_1().child(
                            crate::components::response_viewer::render_response_viewer(
                                _window, cx, &state,
                            ),
                        ))
                        .into_any_element()
                } else {
                    div()
                        .size_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_2xl()
                                .text_color(crate::theme::text_muted())
                                .child("Select a request or test from the sidebar"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_color(crate::theme::text_muted())
                                .child("Or create a new one to get started."),
                        )
                        .into_any_element()
                }),
        )
}

fn render_sidebar(
    cx: &mut App,
    state: &Entity<AppState>,
    active_tab: &crate::state::SidebarTab,
) -> impl IntoElement {
    let (folders, requests, tests, sel_req, sel_test) = {
        let s = state.read(cx);
        let proj = s.active_project.as_ref();
        let f = proj.map(|p| p.folders.clone()).unwrap_or_default();
        let r = proj.map(|p| p.requests.clone()).unwrap_or_default();
        let t = proj.map(|p| p.tests.clone()).unwrap_or_default();
        let sr = proj.and_then(|p| p.selected_request_id.clone());
        let st = proj.and_then(|p| p.selected_test_id.clone());
        (f, r, t, sr, st)
    };

    let is_req = *active_tab == crate::state::SidebarTab::Requests;
    let is_test = *active_tab == crate::state::SidebarTab::Tests;

    div()
        .w(px(260.))
        .h_full()
        .bg(crate::theme::bg_dark())
        .border_r_1()
        .border_color(crate::theme::border())
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .h(px(40.))
                .border_b_1()
                .border_color(crate::theme::border())
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .bg(if is_req {
                            crate::theme::bg_mid()
                        } else {
                            crate::theme::bg_dark()
                        })
                        .text_color(if is_req {
                            crate::theme::text_primary()
                        } else {
                            crate::theme::text_secondary()
                        })
                        .child("Requests"),
                )
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .bg(if is_test {
                            crate::theme::bg_mid()
                        } else {
                            crate::theme::bg_dark()
                        })
                        .text_color(if is_test {
                            crate::theme::text_primary()
                        } else {
                            crate::theme::text_secondary()
                        })
                        .child("Tests"),
                ),
        )
        .child(
            div()
                .flex_1()
                
                .px_2()
                .py_2()
                .child(if is_req {
                    render_requests_list(&folders, &requests, &sel_req).into_any_element()
                } else {
                    render_tests_list(&tests, &sel_test).into_any_element()
                }),
        )
}

fn render_requests_list(
    folders: &[nd_db::models::Folder],
    requests: &[nd_db::models::Request],
    selected_id: &Option<String>,
) -> impl IntoElement {
    let mut children: Vec<AnyElement> = Vec::new();

    children.push(
        div()
            .px_3()
            .py_2()
            .text_sm()
            .text_color(crate::theme::green())
            .child("+ New Request")
            .into_any_element(),
    );

    for req in requests.iter().filter(|r| r.folder_id.is_none()) {
        let is_sel = Some(&req.id) == selected_id.as_ref();
        let color = crate::theme::method_color(&req.method);
        children.push(
            div()
                .pl_6()
                .pr_2()
                .py_1()
                .flex()
                .items_center()
                .gap_2()
                .rounded_md()
                .bg(if is_sel {
                    crate::theme::bg_light()
                } else {
                    crate::theme::bg_dark()
                })
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(color)
                        .w(px(32.))
                        .child(req.method.to_uppercase()),
                )
                .child(div().text_sm().child(req.name.clone()))
                .into_any_element(),
        );
    }

    for folder in folders {
        children.push(
            div()
                .px_3()
                .py_1()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(crate::theme::text_secondary())
                .child(format!("📁 {}", folder.name))
                .into_any_element(),
        );

        for req in requests
            .iter()
            .filter(|r| r.folder_id.as_deref() == Some(&folder.id))
        {
            let is_sel = Some(&req.id) == selected_id.as_ref();
            let color = crate::theme::method_color(&req.method);
            children.push(
                div()
                    .pl_8()
                    .pr_2()
                    .py_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .rounded_md()
                    .bg(if is_sel {
                        crate::theme::bg_light()
                    } else {
                        crate::theme::bg_dark()
                    })
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(color)
                            .w(px(32.))
                            .child(req.method.to_uppercase()),
                    )
                    .child(div().text_sm().child(req.name.clone()))
                    .into_any_element(),
            );
        }
    }

    div().flex().flex_col().children(children)
}

fn render_tests_list(
    tests: &[nd_db::models::Test],
    selected_id: &Option<String>,
) -> impl IntoElement {
    let mut children: Vec<AnyElement> = Vec::new();

    children.push(
        div()
            .px_3()
            .py_2()
            .text_sm()
            .text_color(crate::theme::green())
            .child("+ New Test")
            .into_any_element(),
    );

    for test in tests {
        let is_sel = Some(&test.id) == selected_id.as_ref();
        children.push(
            div()
                .px_3()
                .py_1()
                .flex()
                .items_center()
                .gap_2()
                .rounded_md()
                .bg(if is_sel {
                    crate::theme::bg_light()
                } else {
                    crate::theme::bg_dark()
                })
                .child(div().text_sm().child("🧪"))
                .child(div().text_sm().child(test.name.clone()))
                .into_any_element(),
        );
    }

    div().flex().flex_col().children(children)
}

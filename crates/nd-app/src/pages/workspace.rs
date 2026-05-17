//! Workspace chrome — navigator, duplex editor canvas, inspector, and bottom docks.

use gpui::{
    div, px, AnyElement, App, ClickEvent, Entity, IntoElement, ParentElement, SharedString, Styled,
    Window,
};
use gpui_component::{
    button::{Button, ButtonGroup, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    h_flex,
    label::Label,
    separator::Separator,
    tab::{Tab, TabBar},
    v_flex, ActiveTheme as _, Selectable as _, StyledExt as _,
};

use crate::{
    components::{request_editor, response_viewer, test_editor},
    project_tasks,
    state::{AppState, SidebarTab},
};

fn navigator_menu(
    snapshot: &crate::state::ActiveProject,
    relay: Entity<AppState>,
    folder_tone: gpui::Hsla,
) -> AnyElement {
    return match snapshot.selected_tab {
        SidebarTab::Requests => request_stack(snapshot, relay, folder_tone).into_any_element(),
        SidebarTab::Tests => test_stack(snapshot, relay).into_any_element(),
    };
}

fn request_stack(
    snapshot: &crate::state::ActiveProject,
    relay: Entity<AppState>,
    folder_tone: gpui::Hsla,
) -> impl IntoElement {
    let mut rails: Vec<AnyElement> = Vec::new();

    for orphan in snapshot
        .requests
        .iter()
        .filter(|needle| needle.folder_id.is_none())
        .cloned()
    {
        let token = orphan.id.clone();
        rails.push(
            Button::new(SharedString::from(format!("navigator-req-{token}")))
                .ghost()
                .compact()
                .selected(snapshot.selected_request_id.as_deref() == Some(token.as_str()))
                .label(format!("{}  {}", orphan.method.to_uppercase(), orphan.name))
                .on_click({
                    let shuttle = relay.clone();
                    move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                        let bridge = shuttle.clone();
                        let _ignored = bridge.update(app, |canvas, ctx| {
                            canvas.select_request(Some(token.clone()));
                            canvas.set_sidebar_tab(SidebarTab::Requests);
                            ctx.notify();
                        });
                    }
                })
                .into_any_element(),
        );
    }

    for vault in snapshot.folders.iter().cloned() {
        rails.push(
            div()
                .text_sm()
                .font_semibold()
                .text_color(folder_tone)
                .mb_3()
                .child(format!("📁 {}", vault.name))
                .into_any_element(),
        );

        for child in snapshot
            .requests
            .iter()
            .filter(|needle| needle.folder_id.as_deref() == Some(vault.id.as_str()))
            .cloned()
        {
            let pointer = child.id.clone();

            rails.push(
                Button::new(SharedString::from(format!("navigator-req-{pointer}")))
                    .ghost()
                    .compact()
                    .ml(px(14.))
                    .selected(snapshot.selected_request_id.as_deref() == Some(pointer.as_str()))
                    .label(format!("{}  {}", child.method.to_uppercase(), child.name))
                    .on_click({
                        let shuttle = relay.clone();
                        move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                            let bridge = shuttle.clone();
                            let _ignored = bridge.update(app, |canvas, ctx| {
                                canvas.select_request(Some(pointer.clone()));
                                canvas.set_sidebar_tab(SidebarTab::Requests);
                                ctx.notify();
                            });
                        }
                    })
                    .into_any_element(),
            );
        }
    }

    return v_flex().gap(px(10.)).children(rails);
}

fn test_stack(snapshot: &crate::state::ActiveProject, relay: Entity<AppState>) -> impl IntoElement {
    let mut rails: Vec<AnyElement> = Vec::new();

    for specimen in snapshot.tests.iter().cloned() {
        let fingerprint = specimen.id.clone();

        rails.push(
            Button::new(SharedString::from(format!("navigator-test-{fingerprint}")))
                .ghost()
                .compact()
                .selected(snapshot.selected_test_id.as_deref() == Some(fingerprint.as_str()))
                .label(specimen.name.clone())
                .on_click({
                    let shuttle = relay.clone();
                    move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                        let bridge = shuttle.clone();
                        let _ignored = bridge.update(app, |canvas, ctx| {
                            canvas.select_test(Some(fingerprint.clone()));
                            canvas.set_sidebar_tab(SidebarTab::Tests);
                            ctx.notify();
                        });
                    }
                })
                .into_any_element(),
        );
    }

    return v_flex().gap(px(10.)).children(rails);
}

fn inspector_column(cx: &mut App, relay: &Entity<AppState>) -> impl IntoElement {
    let Some(snapshot) = relay.read(cx).active_project.clone() else {
        return div().flex().items_center().justify_center();
    };

    let subject = snapshot
        .selected_request_id
        .as_deref()
        .and_then(|needle| snapshot.requests.iter().find(|row| row.id == needle));

    let headline_method = SharedString::from(
        subject
            .map(|row| row.method.to_uppercase())
            .unwrap_or_else(|| "—".into()),
    );

    let address = SharedString::from(
        subject
            .map(|row| row.url.clone())
            .unwrap_or_else(|| "".into()),
    );

    let environment = SharedString::from(
        snapshot
            .active_environment_id
            .as_deref()
            .and_then(|needle| snapshot.environments.iter().find(|row| row.id == needle))
            .map(|row| row.name.clone())
            .unwrap_or_else(|| "Base variables".into()),
    );

    let summary_hint = subject
        .map(|found| found.summary.clone())
        // .filter(|blur| !blur.is_empty())
        .unwrap_or_else(|| "Attach a navigator request to hydrate field metadata.".into());

    v_flex()
        .w(px(296.))
        .min_h_full()
        .border_l_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().group_box)
        .px_8()
        .py_10()
        .gap_6()
        .child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(SharedString::from("Inspector")),
        )
        .child(
            GroupBox::new()
                .title(headline_method.clone())
                .outline()
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(summary_hint.clone()),
                ),
        )
        .child(GroupBox::new().title("Targets").outline().children([
            metadata_pair(
                "METHOD",
                headline_method.clone(),
                cx.theme().muted_foreground,
                cx.theme().foreground,
            ),
            metadata_pair(
                "URL",
                address.clone(),
                cx.theme().muted_foreground,
                cx.theme().foreground,
            ),
            metadata_pair(
                "ENVIRONMENT",
                environment.clone(),
                cx.theme().muted_foreground,
                cx.theme().muted_foreground,
            ),
        ]))
}

fn metadata_pair(
    caption: &str,
    value: SharedString,
    caption_colour: gpui::Hsla,
    value_colour: gpui::Hsla,
) -> AnyElement {
    div()
        .flex()
        .justify_between()
        .items_center()
        .gap(px(24.))
        .child(
            div()
                .text_sm()
                .text_color(caption_colour)
                .child(SharedString::from(caption.to_ascii_uppercase())),
        )
        .child(Label::new(value.clone()).text_sm().text_color(value_colour))
        .into_any_element()
}

/// Render KYOSHI-like rails around the duplex editor canvases plus bottom dock stubs.
pub fn render_workspace(
    window: &mut Window,
    cx: &mut App,
    relay: Entity<AppState>,
) -> impl IntoElement {
    let snapshot = relay.read(cx).active_project.clone();
    let Some(surface) = snapshot else {
        return div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(Label::new(SharedString::from(
                "Navigator missing — reopen onboarding.",
            )));
    };

    let palette = relay.clone();
    let command_palette = relay.clone();

    v_flex()
        .size_full()
        .bg(cx.theme().background)
        .text_color(cx.theme().foreground)
        .child(workspace_title_strip(cx, &surface, relay.clone()))
        .child(
            h_flex()
                .flex_1()
                .min_h(px(0.))
                .child(navigator_sidebar(
                    surface.clone(),
                    window,
                    cx,
                    palette.clone(),
                ))
                .child(editor_stack(window, cx, &surface, relay.clone()))
                .child(inspector_column(cx, &relay.clone())),
        )
        .child(bottom_dock(
            cx,
            surface.bottom_panel_tab,
            command_palette.clone(),
        ))
}

fn workspace_title_strip(
    cx: &mut App,
    surface: &crate::state::ActiveProject,
    relay: Entity<AppState>,
) -> impl IntoElement {
    h_flex()
        .items_center()
        .justify_between()
        .w_full()
        .px_8()
        .py_5()
        .border_b_1()
        .border_color(cx.theme().border)
        .child(
            h_flex().items_center().gap_3().child(
                Button::new("workspace-return")
                    .ghost()
                    .compact()
                    .label("Back")
                    .on_click({
                        let shuttle = relay.clone();
                        move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                            let bridge = shuttle.clone();
                            let _ignored = bridge.update(app, |canvas, ctx| {
                                canvas.navigate_to_landing();
                                ctx.notify();
                            });
                        }
                    }),
            ),
        )
        .child(
            h_flex()
                .items_center()
                .gap_4()
                .justify_end()
                .flex_wrap()
                .child(
                    Label::new(SharedString::from(surface.project.name.clone()))
                        .text_lg()
                        .font_semibold(),
                )
                .child(
                    Button::new("workspace-new-request-strip")
                        .primary()
                        .compact()
                        .label("+ New request")
                        .on_click({
                            let shuttle = relay.clone();
                            move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                                project_tasks::spawn_insert_skeleton_request(shuttle.clone(), app);
                            }
                        }),
                ),
        )
}

fn navigator_sidebar(
    snapshot: crate::state::ActiveProject,
    _window: &mut Window,
    cx: &mut App,
    relay: Entity<AppState>,
) -> impl IntoElement {
    v_flex()
        .w(px(280.))
        .min_h_full()
        .border_r_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().secondary)
        .child(
            v_flex()
                .px_6()
                .py_5()
                .gap_6()
                .child(
                    Label::new(SharedString::from(snapshot.project.name.clone()))
                        .text_lg()
                        .font_semibold(),
                )
                .child(
                    Label::new(SharedString::from(format!(
                        "{}",
                        snapshot.db_path.display()
                    )))
                    .text_sm()
                    .text_color(cx.theme().muted_foreground),
                ),
        )
        .child(
            v_flex()
                .px_4()
                .gap_6()
                .flex_1()
                .child(
                    ButtonGroup::new("navigator-switch")
                        .outline()
                        .compact()
                        .child(
                            Button::new("navigator-requests-switch")
                                .label("Requests")
                                .selected(snapshot.selected_tab == SidebarTab::Requests),
                        )
                        .child(
                            Button::new("navigator-tests-switch")
                                .label("Tests")
                                .selected(snapshot.selected_tab == SidebarTab::Tests),
                        )
                        .on_click({
                            let handle = relay.clone();
                            move |indices: &Vec<usize>, _: &mut Window, launcher: &mut App| {
                                let tab = indices.first().copied();
                                match tab {
                                    Some(0) => {
                                        let shuttle = handle.clone();
                                        let _ignored = shuttle.update(launcher, |canvas, ctx| {
                                            canvas.set_sidebar_tab(SidebarTab::Requests);
                                            ctx.notify();
                                        });
                                    }
                                    Some(1) => {
                                        let shuttle = handle.clone();
                                        let _ignored = shuttle.update(launcher, |canvas, ctx| {
                                            canvas.set_sidebar_tab(SidebarTab::Tests);
                                            ctx.notify();
                                        });
                                    }
                                    _ => (),
                                }
                            }
                        }),
                )
                .child(
                    Button::new("navigator-draft-button")
                        .ghost()
                        .compact()
                        .label("+ New HTTP request")
                        .on_click({
                            let shuttle = relay.clone();
                            move |_event: &ClickEvent, _: &mut Window, app: &mut App| {
                                project_tasks::spawn_insert_skeleton_request(shuttle.clone(), app);
                            }
                        }),
                )
                .child(navigator_menu(
                    &snapshot,
                    relay.clone(),
                    cx.theme().muted_foreground,
                )),
        )
}

fn editor_stack(
    window: &mut Window,
    cx: &mut App,
    surface: &crate::state::ActiveProject,
    relay: Entity<AppState>,
) -> impl IntoElement {
    if surface.selected_test_id.is_some() {
        div()
            .flex_1()
            .flex()
            .min_h(px(480.))
            .child(test_editor::render_test_editor(window, cx, &relay))
            .into_any_element()
    } else {
        v_flex()
            .flex_1()
            .min_w(px(560.))
            .min_h_full()
            .overflow_hidden()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .min_h(px(340.))
                    .child(request_editor::render_request_editor(window, cx, &relay)),
            )
            .child(Separator::horizontal())
            .child(
                div()
                    .flex_1()
                    .flex()
                    .min_h(px(320.))
                    .child(response_viewer::render_response_viewer(window, cx, &relay)),
            )
            .into_any_element()
    }
}

fn bottom_dock(cx: &mut App, selected_ix: usize, relay: Entity<AppState>) -> impl IntoElement {
    v_flex()
        .border_t_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().background)
        .child(
            TabBar::new("bottom-shell-tabs")
                .underline()
                .selected_index(selected_ix)
                .on_click({
                    let shuttle = relay.clone();
                    move |index: &usize, _: &mut Window, app: &mut App| {
                        let bridge = shuttle.clone();
                        let _ignored = bridge.update(app, |canvas, ctx| {
                            canvas.set_workspace_bottom_tab(*index);
                            ctx.notify();
                        });
                    }
                })
                .child(Tab::new().label("Logs"))
                .child(Tab::new().label("Console"))
                .child(Tab::new().label("Results")),
        )
        .child(
            Label::new(SharedString::from("Ready."))
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .px_12()
                .py_10(),
        )
}

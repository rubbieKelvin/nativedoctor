// use std::path::PathBuf;

use gpui::{
    App, ClickEvent, Entity, IntoElement, ParentElement, SharedString, Styled,
    Window,
};

use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    // sidebar::SidebarHeader,
    v_flex,
    ActiveTheme as _,
    StyledExt as _,
};

use crate::state::AppState;

// fn recent_rail(
//     cx: &mut App,
//     state: Entity<AppState>,
//     recent_rows: Vec<nd_db::models::RecentProject>,
// ) -> impl IntoElement {
//     let baseline = cx.theme();

//     let body: Vec<AnyElement> = if recent_rows.is_empty() {
//         vec![Label::new(SharedString::from(
//             "Nothing synced yet — create a SQLite workspace to populate this rail.",
//         ))
//         .text_sm()
//         .text_color(baseline.muted_foreground)
//         .into_any_element()]
//     } else {
//         recent_rows
//             .into_iter()
//             .enumerate()
//             .map(|(ix, rp)| {
//                 let disk_path = rp.db_path.clone();
//                 let navigator = state.clone();

//                 Button::new(SharedString::from(format!("recent-project-{ix}")))
//                     .outline()
//                     .compact()
//                     .label(rp.name)
//                     .child(
//                         Label::new(disk_path.clone())
//                             .text_xs()
//                             .text_color(baseline.muted_foreground),
//                     )
//                     .on_click(move |_event: &ClickEvent, _: &mut Window, app| {
//                         crate::project_tasks::spawn_open_database(
//                             navigator.clone(),
//                             PathBuf::from(disk_path.clone()),
//                             app,
//                         );
//                     })
//                     .into_any_element()
//             })
//             .collect()
//     };

//     return v_flex()
//         .w(px(416.))
//         .min_h_full()
//         .border_l_1()
//         .border_color(cx.theme().border)
//         .p_12()
//         .gap_8()
//         .child(
//             SidebarHeader::new()
//                 .justify_between()
//                 .child(Label::new(SharedString::from("RECENT PROJECTS")).text_xs()),
//         )
//         .child(v_flex().gap_6().children(body));
// }

pub fn render_landing(
    _window: &mut Window,
    cx: &mut App,
    state: Entity<AppState>,
) -> impl IntoElement {
    let palette = cx.theme();
    let _recent_rows = state.read(cx).recent_projects.clone();

    let create_handle = state.clone();

    let hero = v_flex()
        .justify_center()
        .items_center()
        .gap_8()
        .flex_1()
        .px_12()
        .child(
            h_flex()
                .justify_center()
                .items_center()
                .gap_3()
                .child(Label::new(SharedString::from("🏥")).text_2xl())
                .child(
                    v_flex().gap_2().justify_center().child(
                        Label::new(SharedString::from("NativeDoctor"))
                            .text_2xl()
                            .font_semibold()
                            .text_color(cx.theme().primary),
                    ).child(
                        Label::new(SharedString::from(
                            "SQLite-backed workspaces, blazing GPUI shell, Rhai scripting, surgical HTTP ergonomics.",
                        )),
                    ),
                ),
        )
        .child(
            h_flex().gap_3().flex_wrap().justify_center().child(
                Button::new("landing-create-db")
                    .primary()
                    .label("Create new project")
                    .on_click({
                        let navigator = create_handle.clone();
                        move |_event: &ClickEvent, _: &mut Window, launcher| {
                            let maybe_paths =
                                crate::components::project_dialog::default_new_project_candidate();
                            let Some((name, disk)) = maybe_paths else {
                                tracing::warn!("Unable to compute onboarding path snapshot");
                                return;
                            };

                            crate::project_tasks::spawn_create_database(navigator.clone(), name, disk, launcher);
                        }
                    }),
            ).child(
                Button::new("landing-open-db")
                    .outline()
                    .label("Open SQLite project…")
                    .on_click(move |_event: &ClickEvent, _: &mut Window, _launcher| {
                        // let maybe_path = pick_database_path();

                        // let Some(selection) = maybe_path else {
                        //     return;
                        // };

                        // crate::project_tasks::spawn_open_database(open_handle.clone(), selection, launcher);
                    }),
            ),
        );

    return h_flex()
        .size_full()
        .bg(palette.background)
        .text_color(palette.foreground)
        .child(hero);
    // .child(recent_rail(cx, state.clone(), recent_rows));
}

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariant, ButtonVariants},
    input::{Input, InputState},
    ActiveTheme, Icon, IconName,
};

use crate::windows::workspace::{TabKind, WorkspaceView};

pub fn render(
    cx: &mut App,
    input: &Entity<InputState>,
    workspace_view: WeakEntity<WorkspaceView>,
) -> impl IntoElement {
    let theme = cx.theme();

    return div()
        .min_h(px(40.))
        .max_h(px(40.))
        .gap_2()
        .flex()
        .flex_row()
        .items_center()
        .justify_center()
        .border_b(px(1.))
        .border_color(theme.border)
        .child(
            Input::new(input)
                .prefix(Icon::new(IconName::Search))
                .appearance(false),
        )
        .child(
            Button::new("add-file")
                .icon(Icon::new(IconName::Plus))
                .with_variant(ButtonVariant::Ghost)
                .h_full()
                .w_10()
                .border_l(px(1.))
                .border_color(theme.border)
                .rounded_none()
                .on_click(move |_event, window, cx| {
                    if let Some(v) = workspace_view.upgrade() {
                        let is_sequence = v.read(cx).active_sidebar_pane == 1;
                        let project_entity = v.read(cx).project.clone();

                        // Create scratch file in-memory (no disk write, no dialog)
                        let uuid = project_entity.update(cx, |opt, _cx| {
                            opt.as_mut()
                                .map(|p| p.create_scratch_file(is_sequence))
                                .unwrap_or_default()
                        });

                        if uuid.is_empty() {
                            return;
                        }

                        v.update(cx, |this, cx| {
                            this.rebuild_trees_from_manifest(cx);

                            let kind = if is_sequence {
                                TabKind::Sequence
                            } else {
                                TabKind::Request
                            };
                            let prefix = if is_sequence {
                                "scratch:sequence:"
                            } else {
                                "scratch:request:"
                            };
                            let id: SharedString =
                                format!("{prefix}{uuid}").into();

                            let label: SharedString = "Untitled".into();
                            this.open_tab(id, label, kind, window, cx);
                            cx.notify();
                        });
                    }
                }),
        );
}
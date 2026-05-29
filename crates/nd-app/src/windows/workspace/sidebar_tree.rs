use std::collections::HashMap;

use gpui::*;
use gpui_component::{
    list::ListItem,
    tab::{Tab, TabBar},
    tree::{tree, TreeState},
    ActiveTheme, IconName, Selectable, Sizable,
};
use nd_core::model::{project::ResourceType, request::RequestFile, sequence::SequenceFile};

use crate::windows::workspace::{sidebar_requests, sidebar_sequences, TabKind, WorkspaceView};

/// Render the sidebar tree area with a tab bar at the bottom for switching
/// between request and sequence views.
///
/// Clicking a leaf item opens a tab in the main panel via `workspace_view`.
pub fn render(
    active_pane: usize,
    workspace_view: WeakEntity<WorkspaceView>,
    requests_state: &Entity<TreeState>,
    sequences_state: &Entity<TreeState>,
    request_meta_map: HashMap<String, RequestFile>,
    sequence_meta_map: HashMap<String, SequenceFile>,
    cx: &mut App,
) -> impl IntoElement {
    let theme = cx.theme().clone();
    let active = active_pane;

    let tree_view = workspace_view.clone();
    let request_meta_map = request_meta_map.clone();
    let sequence_meta_map = sequence_meta_map.clone();

    return div()
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .child({
            let request_meta = request_meta_map.clone();
            let sequence_meta = sequence_meta_map;
            let view = workspace_view.clone();

            match active {
                0 => tree(requests_state, move |ix, entry, selected, _, cx| {
                    let item =
                        sidebar_requests::render_tree_row(ix, entry, selected, cx, &request_meta);
                    return attach_click(item, entry, &view);
                })
                .flex_1()
                .min_h_0()
                .into_any_element(),

                _ => tree(sequences_state, move |ix, entry, selected, _, cx| {
                    let item =
                        sidebar_sequences::render_tree_row(ix, entry, selected, cx, &sequence_meta);
                    return attach_click(item, entry, &view);
                })
                .flex_1()
                .min_h_0()
                .into_any_element(),
            }
        })
        .child(
            TabBar::new("sidebar-tabs")
                .min_h(px(40.))
                .max_h(px(40.))
                .p_0()
                .m_0()
                .border_t(px(1.))
                .border_color(theme.border)
                .with_variant(gpui_component::tab::TabVariant::Underline)
                .selected_index(active as usize)
                .child(
                    Tab::new().small().h_full().selected(active == 0).child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_1()
                            .child(IconName::ExternalLink)
                            .child("requests"),
                    ),
                )
                .child(
                    Tab::new().small().h_full().selected(active == 1).child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_1()
                            .child(IconName::Play)
                            .child("sequences"),
                    ),
                )
                .on_click(move |index, _, app_cx| {
                    if let Some(v) = tree_view.upgrade() {
                        v.update(app_cx, |this: &mut WorkspaceView, cx| {
                            this.active_sidebar_pane = *index;
                            cx.notify();
                        });
                    }
                }),
        );
}

/// attach a click handler to a tree row so that leaf items open a
/// content tab in the main panel.
fn attach_click(
    item: ListItem,
    entry: &gpui_component::tree::TreeEntry,
    view: &WeakEntity<WorkspaceView>,
) -> ListItem {
    if entry.is_folder() {
        return item;
    }

    let item_id = entry.item().id.clone();
    let item_label = entry.item().label.clone();
    let view = view.clone();

    return item.on_click(move |_ev, window, app_cx| {
        if let Some(v) = view.upgrade() {
            let kind = ResourceType::from_id(&item_id)
                .map(|rt| match rt {
                    ResourceType::Environment => TabKind::Environment,
                    ResourceType::Request => TabKind::Request,
                    ResourceType::Sequence => TabKind::Sequence,
                })
                .unwrap_or(TabKind::Request);
            v.update(app_cx, |this, cx| {
                this.open_tab(item_id.clone(), item_label.clone(), kind, window, cx);
            });
        }
    });
}

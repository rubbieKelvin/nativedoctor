use std::collections::HashMap;

use gpui::{div, px, App, Hsla, ParentElement, SharedString, Styled};
use gpui_component::{h_flex, list::ListItem, ActiveTheme, Icon, IconName, Sizable, StyledExt};
use nd_core::model::sequence::SequenceFile;

/// Look up sequence metadata by the tree-item id.
///
/// The id format is `"sequence:<relative-path>"`. The map is keyed by the
/// sequence file's relative path.
pub fn sequence_meta(
    id: &SharedString,
    meta_map: &HashMap<String, SequenceFile>,
) -> Option<SequenceFile> {
    let key = id.strip_prefix("sequence:")?;
    return meta_map.get(key).cloned();
}

fn sequence_colors(cx: &App) -> (Hsla, Hsla) {
    let theme = cx.theme();
    (theme.green.opacity(0.2), theme.green)
}

/// Render a single row in the sequences sidebar tree.
///
/// `meta_map` provides the step count for each sequence; pass an empty map
/// if no project is loaded.
pub fn render_tree_row(
    ix: usize,
    entry: &gpui_component::tree::TreeEntry,
    selected: bool,
    cx: &App,
    meta_map: &HashMap<String, SequenceFile>,
) -> ListItem {
    let item = entry.item();
    let depth = entry.depth();
    let indent = px(16.) * depth + px(8.);

    if entry.is_folder() {
        let folder_icon = if entry.is_expanded() {
            IconName::FolderOpen
        } else {
            IconName::Folder
        };
        let chevron = if entry.is_expanded() {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        };

        return ListItem::new(ix)
            .w_full()
            .selected(selected)
            .pl(indent)
            .child(
                h_flex()
                    .gap_1p5()
                    .items_center()
                    .min_w_0()
                    .child(
                        Icon::new(chevron)
                            .xsmall()
                            .text_color(cx.theme().muted_foreground),
                    )
                    .child(Icon::new(folder_icon).small())
                    .child(div().text_sm().truncate().child(item.label.clone())),
            );
    }

    if let Some(meta) = sequence_meta(&item.id, meta_map) {
        let (bg, fg) = sequence_colors(cx);

        return ListItem::new(ix)
            .w_full()
            .selected(selected)
            .pl(indent)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .min_w_0()
                    .child(Icon::new(IconName::Play).size(px(14.)).text_color(fg))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_sm()
                            .truncate()
                            .child(item.label.clone()),
                    )
                    .child(
                        div()
                            .flex_shrink_0()
                            .px_1p5()
                            .py_0p5()
                            .rounded(px(4.))
                            .bg(bg)
                            .text_xs()
                            .font_semibold()
                            .text_color(fg)
                            .child(format!("{}", meta.groups.len())),
                    ),
            );
    }

    return ListItem::new(ix)
        .w_full()
        .selected(selected)
        .pl(indent)
        .child(
            h_flex()
                .gap_2()
                .items_center()
                .child(Icon::new(IconName::File).small())
                .child(item.label.clone()),
        );
}

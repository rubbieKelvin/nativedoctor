use std::collections::HashMap;

use gpui::{div, px, App, Hsla, IntoElement, ParentElement, SharedString, Styled};
use gpui_component::{h_flex, list::ListItem, ActiveTheme, Icon, IconName, Sizable, StyledExt};
use nd_core::model::request::RequestFile;

/// Look up request metadata by the tree-item id.
///
/// The id format is `"request:<relative-path>"`. The map is keyed by the
/// request file's relative path (e.g. `"requests/request1.json"`).
pub fn request_meta(
    id: &SharedString,
    meta_map: &HashMap<String, RequestFile>,
) -> Option<RequestFile> {
    let key = id.strip_prefix("request:")?;
    return meta_map.get(key).cloned();
}

fn method_colors(method: &str, cx: &App) -> (Hsla, Hsla) {
    let theme = cx.theme();
    match method {
        "GET" => (theme.blue.opacity(0.2), theme.blue),
        "POST" => (theme.yellow.opacity(0.2), theme.yellow),
        "PUT" => (theme.cyan.opacity(0.2), theme.cyan),
        "DEL" | "DELETE" => (theme.red.opacity(0.2), theme.red),
        _ => (theme.muted.opacity(0.25), theme.muted_foreground),
    }
}

/// Render a small coloured badge showing the HTTP method.
pub fn method_badge(method: &str, cx: &App) -> impl IntoElement {
    let (bg, fg) = method_colors(method, cx);

    div()
        .flex_shrink_0()
        .px_1p5()
        .py_0p5()
        .rounded(px(4.))
        .bg(bg)
        .text_xs()
        .font_semibold()
        .text_color(fg)
        .child(SharedString::from(method))
}

/// Render a single row in the requests sidebar tree.
///
/// `meta_map` provides the method + URL for each request; pass an empty map
/// if no project is loaded (all rows fall back to a plain file icon).
pub fn render_tree_row(
    ix: usize,
    entry: &gpui_component::tree::TreeEntry,
    selected: bool,
    cx: &App,
    meta_map: &HashMap<String, RequestFile>,
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

    let label = request_meta(&item.id, meta_map)
        .and_then(|m| Some(SharedString::from(m.label())))
        .unwrap_or(item.label.clone());

    if let Some(meta) = request_meta(&item.id, meta_map) {
        return ListItem::new(ix)
            .w_full()
            .selected(selected)
            .pl(indent)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .min_w_0()
                    .child(method_badge(&meta.request.method.as_str(), cx))
                    .child(div().flex_1().min_w_0().text_sm().truncate().child(label)),
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
                .child(label),
        );
}

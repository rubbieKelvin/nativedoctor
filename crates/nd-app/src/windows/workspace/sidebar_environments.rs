use std::collections::HashMap;

use gpui::{div, px, App, Hsla, ParentElement, SharedString, Styled};
use gpui_component::{
    h_flex, list::ListItem, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};

/// Metadata for a single environment shown in the sidebar tree.
#[derive(Clone)]
pub struct EnvMeta {
    /// Human-readable environment name.
    pub name: SharedString,
    /// Number of variables defined in this environment.
    pub var_count: usize,
}

/// Look up environment metadata by the tree-item id.
///
/// The id format is `"env:<name>"`. The map is keyed by the environment name.
pub fn env_meta(
    id: &SharedString,
    meta_map: &HashMap<String, EnvMeta>,
) -> Option<EnvMeta> {
    let key = id.strip_prefix("env:")?;
    return meta_map.get(key).cloned();
}

fn env_colors(cx: &App) -> (Hsla, Hsla) {
    let theme = cx.theme();
    return (theme.cyan.opacity(0.2), theme.cyan);
}

/// Render a single row in the environments sidebar tree.
///
/// `meta_map` provides the variable count for each environment; pass an empty
/// map if no environments are defined.
pub fn render_tree_row(
    ix: usize,
    entry: &gpui_component::tree::TreeEntry,
    selected: bool,
    cx: &App,
    meta_map: &HashMap<String, EnvMeta>,
) -> ListItem {
    let item = entry.item();

    if let Some(meta) = env_meta(&item.id, meta_map) {
        let (bg, fg) = env_colors(cx);

        return ListItem::new(ix)
            .w_full()
            .selected(selected)
            .px_2()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .min_w_0()
                    .child(
                        Icon::new(IconName::Globe)
                            .size(px(14.))
                            .text_color(fg),
                    )
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
                            .child(format!("{} vars", meta.var_count)),
                    ),
            );
    }

    return ListItem::new(ix)
        .w_full()
        .selected(selected)
        .px_2()
        .child(
            h_flex()
                .gap_2()
                .items_center()
                .child(Icon::new(IconName::File).small())
                .child(item.label.clone()),
        );
}

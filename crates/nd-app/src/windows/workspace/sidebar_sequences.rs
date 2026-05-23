use gpui::{div, px, App, Hsla, ParentElement, SharedString, Styled};
use gpui_component::{
    h_flex, list::ListItem, tree::TreeItem, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};

use super::resources::ResourceType;

#[derive(Clone, Copy)]
pub struct SequenceMeta {
    #[allow(unused)]
    pub name: &'static str,
    pub steps: usize,
}

pub fn sequence_meta(id: &SharedString) -> Option<SequenceMeta> {
    if ResourceType::from_id(id) != Some(ResourceType::Sequence) {
        return None;
    }
    return SAMPLE_SEQUENCES
        .iter()
        .find(|r| r.0 == id.as_str())
        .map(|r| r.1);
}

pub fn sample_sequence_items() -> Vec<TreeItem> {
    return vec![
        TreeItem::new(
            ResourceType::Sequence.make_id("new-user-flow"),
            "Create & verify user",
        )
        .expanded(true)
        .children([
            TreeItem::new(
                ResourceType::Sequence.make_id("post-user"),
                "Create user",
            ),
            TreeItem::new(
                ResourceType::Sequence.make_id("get-customer"),
                "Verify user",
            ),
        ]),
        TreeItem::new(
            ResourceType::Sequence.make_id("order-flow"),
            "Order processing pipeline",
        )
        .children([
            TreeItem::new(
                ResourceType::Sequence.make_id("post-customer"),
                "Create customer",
            ),
            TreeItem::new(
                ResourceType::Sequence.make_id("put-user"),
                "Update order",
            ),
            TreeItem::new(
                ResourceType::Sequence.make_id("del-users"),
                "Cleanup",
            ),
        ]),
    ];
}

static SAMPLE_SEQUENCES: &[(&str, SequenceMeta)] = &[
    (
        "sequence:new-user-flow",
        SequenceMeta {
            name: "Create & verify user",
            steps: 2,
        },
    ),
    (
        "sequence:order-flow",
        SequenceMeta {
            name: "Order processing pipeline",
            steps: 3,
        },
    ),
];

fn sequence_colors(cx: &App) -> (Hsla, Hsla) {
    let theme = cx.theme();
    (theme.green.opacity(0.2), theme.green)
}

pub fn render_tree_row(
    ix: usize,
    entry: &gpui_component::tree::TreeEntry,
    selected: bool,
    cx: &App,
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

    if let Some(meta) = sequence_meta(&item.id) {
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
                            .child(format!("{} steps", meta.steps)),
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

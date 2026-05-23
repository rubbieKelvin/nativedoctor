use gpui::{div, px, App, Hsla, ParentElement, SharedString, Styled};
use gpui_component::{
    h_flex, list::ListItem, tree::TreeItem, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};

const SEQUENCE_PREFIX: &str = "sequence:";

#[derive(Clone, Copy)]
pub struct SequenceMeta {
    #[allow(unused)]
    pub name: &'static str,
    pub steps: usize,
}

pub fn sequence_meta(id: &SharedString) -> Option<SequenceMeta> {
    if !id.starts_with(SEQUENCE_PREFIX) {
        return None;
    }
    return SAMPLE_SEQUENCES
        .iter()
        .find(|r| r.0 == id.as_str())
        .map(|r| r.1);
}

pub fn sample_sequence_items() -> Vec<TreeItem> {
    return vec![
        TreeItem::new("sequence:new-user-flow", "Create & verify user")
            .expanded(true)
            .children([
                TreeItem::new("sequence:post-user", "Create user"),
                TreeItem::new("sequence:get-customer", "Verify user"),
            ]),
        TreeItem::new("sequence:order-flow", "Order processing pipeline").children([
            TreeItem::new("sequence:post-customer", "Create customer"),
            TreeItem::new("sequence:put-user", "Update order"),
            TreeItem::new("sequence:del-users", "Cleanup"),
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

    // Render child requests with a play icon and title
    if item.id.starts_with("sequence:") {
        let (_bg, fg) = sequence_colors(cx);

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
                            .text_color(cx.theme().foreground)
                            .truncate()
                            .child(item.label.clone()),
                    ),
            );
    }

    // Render sequence entries with a play icon and step count badge
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

    // Fallback for unknown items
    ListItem::new(ix)
        .w_full()
        .selected(selected)
        .pl(indent)
        .child(
            h_flex()
                .gap_2()
                .items_center()
                .child(Icon::new(IconName::File).small())
                .child(item.label.clone()),
        )
}

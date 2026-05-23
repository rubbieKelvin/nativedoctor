use gpui::{div, px, App, Hsla, IntoElement, ParentElement, SharedString, Styled};
use gpui_component::{
    h_flex, list::ListItem, tree::TreeItem, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};

use super::resources::ResourceType;

#[derive(Clone, Copy)]
pub struct RequestMeta {
    pub method: &'static str,
    pub url: &'static str,
}

pub fn request_meta(id: &SharedString) -> Option<RequestMeta> {
    if ResourceType::from_id(id) != Some(ResourceType::Request) {
        return None;
    }
    return SAMPLE_REQUESTS
        .iter()
        .find(|r| r.0 == id.as_str())
        .map(|r| r.1);
}

pub fn sample_tree_items() -> Vec<TreeItem> {
    let folder1 = TreeItem::new(ResourceType::Folder.make_id("folder-01"), "Folder 01")
        .expanded(true)
        .children([
            TreeItem::new(ResourceType::Request.make_id("del-users"), "Delete users"),
            TreeItem::new(ResourceType::Request.make_id("put-user"), "Update user"),
            TreeItem::new(ResourceType::Request.make_id("post-user"), "Create user"),
        ]);

    let folder2 = TreeItem::new(ResourceType::Folder.make_id("new-customer"), "New Customer").children([
        TreeItem::new(ResourceType::Request.make_id("get-customer"), "Get customer"),
        TreeItem::new(
            ResourceType::Request.make_id("post-customer"),
            "Create customer",
        ),
    ]);

    return vec![folder1, folder2];
}

static SAMPLE_REQUESTS: &[(&str, RequestMeta)] = &[
    (
        "request:del-users",
        RequestMeta {
            method: "DEL",
            url: "https://dummyjson.com/users/1",
        },
    ),
    (
        "request:put-user",
        RequestMeta {
            method: "PUT",
            url: "https://dummyjson.com/users/1",
        },
    ),
    (
        "request:post-user",
        RequestMeta {
            method: "POST",
            url: "https://dummyjson.com/users/add",
        },
    ),
    (
        "request:get-customer",
        RequestMeta {
            method: "GET",
            url: "https://dummyjson.com/users/2",
        },
    ),
    (
        "request:post-customer",
        RequestMeta {
            method: "POST",
            url: "https://dummyjson.com/users/add",
        },
    ),
];

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

    if let Some(meta) = request_meta(&item.id) {
        return ListItem::new(ix)
            .w_full()
            .selected(selected)
            .pl(indent)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .min_w_0()
                    .child(method_badge(meta.method, cx))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .truncate()
                            .child(meta.url),
                    ),
            );
    }

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

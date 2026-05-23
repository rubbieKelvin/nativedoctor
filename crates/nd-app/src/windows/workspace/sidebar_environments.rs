use gpui::{div, px, App, Hsla, ParentElement, SharedString, Styled};
use gpui_component::{
    h_flex, list::ListItem, tree::TreeItem, ActiveTheme, Icon, IconName, Sizable, StyledExt,
};

const ENV_PREFIX: &str = "env:";

#[derive(Clone, Copy)]
pub struct EnvMeta {
    #[allow(unused)]
    pub name: &'static str,
    pub var_count: usize,
}

pub fn env_meta(id: &SharedString) -> Option<EnvMeta> {
    if !id.starts_with(ENV_PREFIX) {
        return None;
    }
    return SAMPLE_ENVIRONMENTS
        .iter()
        .find(|r| r.0 == id.as_str())
        .map(|r| r.1);
}

pub fn sample_env_items() -> Vec<TreeItem> {
    return SAMPLE_ENVIRONMENTS
        .iter()
        .map(|(id, meta)| TreeItem::new(*id, meta.name))
        .collect();
}

static SAMPLE_ENVIRONMENTS: &[(&str, EnvMeta)] = &[
    (
        "env:development",
        EnvMeta {
            name: "Development",
            var_count: 3,
        },
    ),
    (
        "env:staging",
        EnvMeta {
            name: "Staging",
            var_count: 3,
        },
    ),
    (
        "env:production",
        EnvMeta {
            name: "Production",
            var_count: 2,
        },
    ),
];

fn env_colors(cx: &App) -> (Hsla, Hsla) {
    let theme = cx.theme();
    return (theme.cyan.opacity(0.2), theme.cyan);
}

pub fn render_tree_row(
    ix: usize,
    entry: &gpui_component::tree::TreeEntry,
    selected: bool,
    cx: &App,
) -> ListItem {
    let item = entry.item();
    let (bg, fg) = env_colors(cx);

    if let Some(meta) = env_meta(&item.id) {
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

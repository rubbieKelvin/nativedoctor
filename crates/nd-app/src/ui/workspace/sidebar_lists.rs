use gpui::{Context, Window};
use gpui_component::{list, table::TableState, IndexPath};

struct FileBrowserDelegate {
    files: Vec<FileInfo>,
    selected: Option<IndexPath>,
}

#[derive(Clone)]
struct FileInfo {
    name: String,
    is_directory: bool,
    size: Option<u64>,
}

impl list::ListDelegate for FileBrowserDelegate {
    type Item = list::ListItem;

    fn render_item(
        &mut self,
        ix: IndexPath,
        window: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> Option<Self::Item> {
        self.files.get(ix.row).map(|file| {
            let icon = if file.is_directory {
                IconName::Folder
            } else {
                IconName::File
            };

            ListItem::new(ix)
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .child(
                            h_flex()
                                .items_center()
                                .gap_2()
                                .child(Icon::new(icon))
                                .child(Label::new(file.name.clone())),
                        )
                        .when_some(file.size, |this, size| {
                            this.child(
                                Label::new(format_size(size))
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground),
                            )
                        }),
                )
                .selected(Some(ix) == self.selected)
        })
    }
}

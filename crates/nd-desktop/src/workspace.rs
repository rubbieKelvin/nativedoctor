//! The root view: title bar, sidebar, tabs, main area and status bar.

use std::path::{Path, PathBuf};

use gpui_kit::component::button::*;
use gpui_kit::component::list::ListItem;
use gpui_kit::component::tab::{Tab, TabBar};
use gpui_kit::component::tree::{tree, TreeItem, TreeState};
use gpui_kit::component::*;
use gpui_kit::*;
use nd_constants::REQUEST_SUFFIX;
use nd_core::Collection;

use crate::model::sidebar::tree_items;
use crate::model::tabs::Tabs;

pub struct Workspace {
    collection: Option<Collection>,
    tree: Entity<TreeState>,
    tabs: Tabs,
    error: Option<String>,
}

impl Workspace {
    pub fn new(start: Option<PathBuf>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let tree = cx.new(|cx| TreeState::new(cx));
        let mut workspace = Workspace {
            collection: None,
            tree,
            tabs: Tabs::default(),
            error: None,
        };
        if let Some(path) = start {
            workspace.open_collection(&path, window, cx);
        }
        return workspace;
    }

    /// Opens the collection holding `path`, found by walking up from it.
    pub fn open_collection(&mut self, path: &Path, window: &mut Window, cx: &mut Context<Self>) {
        let start = if path.is_dir() {
            path
        } else {
            path.parent().unwrap_or(path)
        };
        let found = Collection::find(start);
        match found {
            Ok(Some(collection)) => self.show(collection, window, cx),
            Ok(None) => {
                self.error = Some(format!("No nativedoctor.yaml in {} or its parents", start.display()));
            }
            Err(error) => self.error = Some(error.to_string()),
        }
        cx.notify();
    }

    fn show(&mut self, collection: Collection, window: &mut Window, cx: &mut Context<Self>) {
        let items = match collection.tree() {
            Ok(nodes) => tree_items(&nodes),
            Err(error) => {
                self.error = Some(error.to_string());
                return;
            }
        };
        self.error = None;
        self.tabs = Tabs::default();
        self.tree.update(cx, |state, cx| state.set_items(items, cx));
        window.set_window_title(&format!("{} — NativeDoctor", collection.manifest.name));
        self.collection = Some(collection);
    }

    fn pick_folder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let picked = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Open".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = picked.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            this.update_in(cx, |this, window, cx| this.open_collection(&path, window, cx))
                .ok();
        })
        .detach();
    }

    fn open_request(&mut self, path: &Path, cx: &mut Context<Self>) {
        let title = nd_core::RequestFile::load(path)
            .map_or_else(
                |_| nd_core::RequestFile::default().display_name(path),
                |file| file.display_name(path),
            );
        self.tabs.open(path, title);
        cx.notify();
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity().downgrade();
        let theme = cx.theme().clone();
        return div()
            .v_flex()
            .size_full()
            .bg(theme.sidebar)
            .child(
                div()
                    .px_3()
                    .py_2()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child(
                        self.collection
                            .as_ref()
                            .map_or("No collection".to_string(), |c| c.manifest.name.clone()),
                    ),
            )
            .child(
                div().flex_1().min_h_0().child(tree(&self.tree, move |ix, entry, _, _, _| {
                    let item: &TreeItem = entry.item();
                    let id = item.id.clone();
                    let this = this.clone();
                    let icon = if item.is_folder() {
                        IconName::Folder
                    } else {
                        IconName::File
                    };
                    ListItem::new(ix)
                        .pl(px(8. + 14. * entry.depth() as f32))
                        .child(
                            div()
                                .h_flex()
                                .gap_2()
                                .child(Icon::new(icon).size_4())
                                .child(item.label.clone()),
                        )
                        .on_click(move |_, _, cx| {
                            let path = PathBuf::from(id.as_ref());
                            if !path.to_string_lossy().ends_with(REQUEST_SUFFIX) {
                                return;
                            }
                            this.update(cx, |this, cx| this.open_request(&path, cx)).ok();
                        })
                })),
            );
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tabs = self.tabs.items.iter().map(|tab| Tab::new().label(tab.title.clone()));
        return TabBar::new("request-tabs")
            .underline()
            .selected_index(self.tabs.active.unwrap_or(0))
            .children(tabs.collect::<Vec<_>>())
            .on_click(cx.listener(|this, index: &usize, _, cx| {
                this.tabs.select(*index);
                cx.notify();
            }));
    }

    fn render_main(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let body = match self.tabs.active_tab() {
            Some(tab) => div()
                .v_flex()
                .gap_1()
                .child(div().text_lg().child(tab.title.clone()))
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.muted_foreground)
                        .child(tab.path.display().to_string()),
                ),
            None => div()
                .v_flex()
                .gap_1()
                .child(div().text_color(theme.muted_foreground).child("Pick a request from the sidebar")),
        };
        return div()
            .v_flex()
            .size_full()
            .child(self.render_tabs(cx))
            .child(div().flex_1().p_4().child(body));
    }

    fn render_welcome(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let mut column = div()
            .v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_3()
            .child(div().text_2xl().child("NativeDoctor"))
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child("Open a folder with a nativedoctor.yaml"),
            )
            .child(
                Button::new("open-folder")
                    .primary()
                    .label("Open Folder…")
                    .on_click(cx.listener(|this, _, window, cx| this.pick_folder(window, cx))),
            );
        if let Some(error) = &self.error {
            column = column.child(div().text_color(theme.danger).child(error.clone()));
        }
        return column;
    }
}

impl Render for Workspace {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let title = self
            .collection
            .as_ref()
            .map_or("NativeDoctor".to_string(), |c| c.manifest.name.clone());
        let env = self
            .collection
            .as_ref()
            .and_then(|c| c.manifest.default_env.clone());

        let content = if self.collection.is_some() {
            h_resizable("main-split")
                .child(
                    resizable_panel()
                        .size(px(260.))
                        .size_range(px(180.)..px(480.))
                        .child(self.render_sidebar(cx)),
                )
                .child(resizable_panel().child(self.render_main(cx)))
                .into_any_element()
        } else {
            self.render_welcome(cx).into_any_element()
        };

        return div()
            .v_flex()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(
                TitleBar::new().child(
                    div()
                        .h_flex()
                        .w_full()
                        .justify_between()
                        .pr_3()
                        .child(div().text_sm().child(title))
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.muted_foreground)
                                .child(env.map_or(String::new(), |e| format!("env: {e}"))),
                        ),
                ),
            )
            .child(div().flex_1().min_h_0().child(content))
            .child(
                div()
                    .h_flex()
                    .px_3()
                    .py_1()
                    .border_t_1()
                    .border_color(theme.border)
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child(match &self.collection {
                        Some(c) => format!("{}", c.root.display()),
                        None => "Ready".to_string(),
                    }),
            );
    }
}

mod project;
mod sidebar_environments;
mod sidebar_requests;
mod sidebar_search;
mod sidebar_sequences;
pub mod sidebar_tree;

use std::collections::HashMap;
use std::path::PathBuf;

use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    button::{self, ButtonVariants},
    input,
    resizable::{h_resizable, resizable_panel},
    tab::{self, Tab, TabBar},
    tree::{TreeItem, TreeState},
    ActiveTheme, IconName, Selectable, Sizable, Theme,
};
use nd_core::model::project::{ProjectFile, ProjectResource, ResourceType};
use tracing::warn;

use crate::{
    ui::components::{
        self,
        env_panel::EnvPanel,
        env_popup,
        project::recents::{ProjectPopup, ProjectPopupEvents, ProjectPopupState},
        request::RequestPanel,
    },
    windows::app_wrapper,
    windows::workspace::project::OpenProject,
};

#[derive(Clone, PartialEq)]
pub(crate) enum TabKind {
    Request,
    Sequence,
    Environment,
}

#[derive(Clone, PartialEq)]
struct OpenTab {
    id: SharedString,
    label: SharedString,
    kind: TabKind,
}

pub struct WorkspaceView {
    project: Entity<Option<project::OpenProject>>,
    search_input_state: Entity<input::InputState>,
    pub requests_tree_state: Entity<TreeState>,
    pub sequences_tree_state: Entity<TreeState>,
    #[allow(dead_code)]
    env_tree_state: Entity<TreeState>,
    pub active_sidebar_pane: usize,
    env_popup_state: Entity<env_popup::EnvPopupState>,
    project_popup_state: Entity<ProjectPopupState>,
    open_tabs: Vec<OpenTab>,
    active_tab_index: Option<usize>,
    tab_panels: HashMap<String, Entity<RequestPanel>>,
    env_panels: HashMap<String, Entity<EnvPanel>>,
    _subscriptions: Vec<Subscription>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search requests..."));

        let env_popup_state = cx.new(|cx| env_popup::EnvPopupState::new(window, cx));
        let project_popup_state = cx.new(|cx| ProjectPopupState::new(window, cx));

        let requests_tree_state = cx.new(|cx| TreeState::new(cx));
        let sequences_tree_state = cx.new(|cx| TreeState::new(cx));
        let env_tree_state = cx.new(|cx| TreeState::new(cx));

        let project = cx.new(|_cx| None::<project::OpenProject>);

        let _subscriptions = vec![cx.subscribe_in(
            &project_popup_state,
            window,
            |this, _entity, event, window, cx| match event {
                ProjectPopupEvents::OpenProject(path) => {
                    this.handle_open_project(path.clone(), window, cx);
                }
            },
        )];

        return Self {
            project,
            search_input_state,
            requests_tree_state,
            sequences_tree_state,
            env_tree_state,
            active_sidebar_pane: 0,
            env_popup_state,
            project_popup_state,
            open_tabs: Vec::new(),
            active_tab_index: None,
            tab_panels: HashMap::new(),
            env_panels: HashMap::new(),
            _subscriptions,
        };
    }

    fn find_tab_position(&self, id: &SharedString) -> Option<usize> {
        return self.open_tabs.iter().position(|tab| tab.id == *id);
    }

    pub fn open_tab(
        &mut self,
        id: SharedString,
        label: SharedString,
        kind: TabKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(position) = self.find_tab_position(&id) {
            self.active_tab_index = Some(position);
            return;
        }

        self.open_tabs.push(OpenTab {
            id: id.clone(),
            label: label.clone(),
            kind: kind.clone(),
        });
        let new_index = self.open_tabs.len() - 1;

        if kind == TabKind::Request {
            let panel = cx.new(|cx| RequestPanel::new(window, cx));
            self.tab_panels.insert(id.to_string(), panel);
        } else if kind == TabKind::Environment {
            let panel = cx.new(|cx| EnvPanel::new(window, cx));
            self.env_panels.insert(id.to_string(), panel);
        }

        self.active_tab_index = Some(new_index);
    }

    /// Handle an open-project request triggered from the title-bar popover.
    ///
    /// Loads the `nativedoctor.yaml` at `path`, parses it, builds the
    /// in-memory [`OpenProject`], scans request files for metadata, and
    /// populates the sidebar tree states.
    fn handle_open_project(&mut self, path: PathBuf, _window: &mut Window, cx: &mut Context<Self>) {
        let project_file = match ProjectFile::from_file(&path) {
            Ok(pf) => pf,
            Err(e) => {
                tracing::error!("Failed to load project file: {}", e);
                return;
            }
        };

        let mut open_project = OpenProject::from_project_file(project_file.clone(), path.clone());
        open_project.load_resources();

        // Build request tree + metadata from the ProjectResource tree.
        let request_tree_items =
            build_items_from_resources(&project_file.requests, ResourceType::Request);

        self.requests_tree_state.update(cx, |state, cx| {
            state.set_items(request_tree_items, cx);
        });

        // Build sequence tree from the ProjectResource tree.
        let sequence_tree_items =
            build_items_from_resources(&project_file.sequences, ResourceType::Sequence);

        self.sequences_tree_state.update(cx, |state, cx| {
            state.set_items(sequence_tree_items, cx);
        });

        // Store the loaded project and update the UI.
        self.project.update(cx, |proj, cx| {
            *proj = Some(open_project);
            cx.notify();
        });

        cx.notify();
    }

    fn close_tab(&mut self, index: usize, _window: &mut Window, _cx: &mut Context<Self>) {
        if index >= self.open_tabs.len() {
            return;
        }

        let removed = self.open_tabs.remove(index);
        self.tab_panels.remove(removed.id.as_str());
        self.env_panels.remove(removed.id.as_str());

        if self.open_tabs.is_empty() {
            self.active_tab_index = None;
            return;
        }

        self.active_tab_index = Some(match self.active_tab_index {
            Some(active) if active > index => active - 1,
            Some(active) if active == index => index.min(self.open_tabs.len() - 1),
            _ => index,
        });
    }

    fn sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let project = self.project.read(cx);
        let title = match self.active_sidebar_pane {
            0 => "Requests",
            _ => "Sequences",
        };

        let (request, sequence) = if let Some(project) = project {
            (
                project.loaded_requests.clone(),
                project.loaded_sequences.clone(),
            )
        } else {
            (HashMap::new(), HashMap::new())
        };

        return div()
            .flex()
            .flex_col()
            .size_full()
            .child(sidebar_search::render(cx, &self.search_input_state))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .child(title)
                    .text_sm()
                    .text_color(theme.muted_foreground),
            )
            .child(sidebar_tree::render(
                self.active_sidebar_pane,
                cx.weak_entity(),
                &self.requests_tree_state,
                &self.sequences_tree_state,
                request,
                sequence,
                cx,
            ));
    }

    fn render_tab_bar(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_tab_index;
        let view = cx.weak_entity();

        let mut bar = TabBar::new("workspace-tabs")
            .min_h(px(40.))
            .max_h(px(40.))
            .p_0()
            .m_0()
            .with_variant(tab::TabVariant::Underline)
            .when_some(active, |this, idx| this.selected_index(idx))
            .prefix(
                div()
                    .px_2()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .border_r(px(1.))
                    .border_color(theme.border)
                    .child(
                        button::Button::new("back")
                            .icon(IconName::ArrowLeft)
                            .with_variant(button::ButtonVariant::Text),
                    )
                    .child(
                        button::Button::new("forward")
                            .icon(IconName::ArrowRight)
                            .with_variant(button::ButtonVariant::Text),
                    ),
            )
            .border_color(theme.border);

        for (i, tab) in self.open_tabs.iter().enumerate() {
            let tab_id = tab.id.clone();
            let tab_label = tab.label.clone();
            let is_active = active == Some(i);

            let tab_icon = match tab.kind {
                TabKind::Request => IconName::ExternalLink,
                TabKind::Sequence => IconName::Play,
                TabKind::Environment => IconName::Globe,
            };

            let tab_pill = Tab::new()
                .small()
                .h_full()
                .when(is_active, |this| this.selected(true))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(tab_icon)
                        .child(tab_label.clone()),
                )
                .suffix({
                    let tab_view = view.clone();
                    button::Button::new(format!("close-{i}"))
                        .icon(IconName::Close)
                        .xsmall()
                        .with_variant(button::ButtonVariant::Ghost)
                        .on_click(move |_event, window, app_cx| {
                            if let Some(v) = tab_view.upgrade() {
                                v.update(app_cx, |this: &mut WorkspaceView, cx| {
                                    if let Some(pos) = this.find_tab_position(&tab_id) {
                                        this.close_tab(pos, window, cx);
                                    }
                                    cx.notify();
                                });
                            }
                        })
                });

            bar = bar.child(tab_pill);
        }

        bar = bar.on_click(move |index, _, app_cx| {
            if let Some(v) = view.upgrade() {
                v.update(app_cx, |this: &mut WorkspaceView, cx| {
                    this.active_tab_index = Some(*index);
                    cx.notify();
                });
            }
        });

        return bar;
    }

    fn render_tab_content(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let Some(active_index) = self.active_tab_index else {
            return div().flex_1().child("No open tabs").into_any_element();
        };

        let Some(tab) = self.open_tabs.get(active_index) else {
            return div().flex_1().child("No open tabs").into_any_element();
        };

        return match tab.kind {
            TabKind::Request => {
                if let Some(panel) = self.tab_panels.get(tab.id.as_str()) {
                    div()
                        .flex_1()
                        .min_h_0()
                        .child(panel.clone())
                        .into_any_element()
                } else {
                    div()
                        .flex_1()
                        .child("Request panel not found")
                        .into_any_element()
                }
            }
            TabKind::Environment => {
                if let Some(panel) = self.env_panels.get(tab.id.as_str()) {
                    div()
                        .flex_1()
                        .min_h_0()
                        .child(panel.clone())
                        .into_any_element()
                } else {
                    div()
                        .flex_1()
                        .child("Environment panel not found")
                        .into_any_element()
                }
            }
            TabKind::Sequence => div()
                .flex_1()
                .child(format!("Sequence: {}", tab.label))
                .into_any_element(),
        };
    }

    fn mainpanel(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        if self.open_tabs.is_empty() {
            return div().flex_1().flex().items_center().justify_center().child(
                div()
                    .text_base()
                    .text_color(theme.muted_foreground)
                    .child("Select a request or sequence from the sidebar to open it."),
            );
        }

        let tab_bar = self.render_tab_bar(&theme, cx);
        let content = self.render_tab_content(window, cx);

        return div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(tab_bar)
            .child(content);
    }

    fn render_workspace(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl Element {
        let theme = cx.theme().clone();

        return div().flex_1().min_h_0().child(
            div()
                .size_full()
                .bg(theme.background)
                .text_color(theme.foreground)
                .child(
                    h_resizable("sidebar-workspace")
                        .child(
                            resizable_panel()
                                .size(px(384.))
                                .size_range(px(200.)..px(600.))
                                .child(self.sidebar(cx)),
                        )
                        .child(resizable_panel().child(self.mainpanel(window, cx))),
                ),
        );
    }
}

/// recursively walk a [`ProjectResource`] tree and produce a flat list of
/// [`TreeItem`]s filtered to the given [`ResourceType`].
///
/// - `Folder` nodes become expandable folder tree items.
/// - resource leaf nodes are included only when they match `kind`.
fn build_items_from_resources(resources: &[ProjectResource], kind: ResourceType) -> Vec<TreeItem> {
    let mut items = Vec::new();

    for resource in resources {
        match resource {
            ProjectResource::Folder(_, name, children) => {
                let folder_item = TreeItem::new(resource.make_id(), name.clone())
                    .expanded(true)
                    .children(build_items_from_resources(children, kind));

                items.push(folder_item);
            }
            ProjectResource::Request(path) if kind == ResourceType::Request => {
                let label = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                items.push(TreeItem::new(resource.make_id(), label));
            }
            ProjectResource::Sequence(path) if kind == ResourceType::Sequence => {
                let label = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                items.push(TreeItem::new(resource.make_id(), label));
            }

            _ => {
                warn!("You've not handled other resource types!");
            }
        }
    }

    return items;
}

impl Render for WorkspaceView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let no_open_project = self.project.read(cx).is_none();
        let project_title = match self.project.read(cx) {
            Some(project) => project.name.clone(),
            None => "No project".into(),
        };

        return app_wrapper::<Self>(window, cx)
            .child(components::title_bar::render(
                cx,
                ProjectPopup::new(project_title, self.project_popup_state.clone()),
                if no_open_project {
                    None
                } else {
                    Some(env_popup::EnvPopup::new(self.env_popup_state.clone()))
                },
            ))
            .when_else(
                self.project.read(cx).is_some(),
                |el| el.child(self.render_workspace(window, cx)),
                |el| el.child(div().child("No project")),
            );
    }
}

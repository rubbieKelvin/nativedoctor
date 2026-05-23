mod resources;
mod sidebar_environments;
mod sidebar_requests;
mod sidebar_sequences;

use std::collections::HashMap;

use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    button::{self, ButtonVariants},
    input,
    resizable::{h_resizable, resizable_panel},
    tab::{self, Tab, TabBar},
    tree::{tree, TreeState},
    ActiveTheme, Icon, IconName, Selectable, Sizable, Theme,
};

use crate::{
    ui::components::{self, env_panel::EnvPanel, env_popup, request::RequestPanel},
    windows::app_wrapper,
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
    search_input_state: Entity<input::InputState>,
    requests_tree_state: Entity<TreeState>,
    sequences_tree_state: Entity<TreeState>,
    env_tree_state: Entity<TreeState>,
    active_sidebar_pane: usize,
    env_popup_state: Entity<env_popup::EnvPopupState>,
    open_tabs: Vec<OpenTab>,
    active_tab_index: Option<usize>,
    tab_panels: HashMap<String, Entity<RequestPanel>>,
    env_panels: HashMap<String, Entity<EnvPanel>>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search requests..."));

        let env_popup_state = cx.new(|cx| env_popup::EnvPopupState::new(window, cx));

        let requests_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_requests::sample_tree_items()));
        let sequences_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_sequences::sample_sequence_items()));
        let env_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_environments::sample_env_items()));

        return Self {
            search_input_state,
            requests_tree_state,
            sequences_tree_state,
            env_tree_state,
            active_sidebar_pane: 0,
            env_popup_state,
            open_tabs: Vec::new(),
            active_tab_index: None,
            tab_panels: HashMap::new(),
            env_panels: HashMap::new(),
        };
    }

    fn find_tab_position(&self, id: &SharedString) -> Option<usize> {
        return self.open_tabs.iter().position(|tab| tab.id == *id);
    }

    fn open_tab(
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

    fn sidebar(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let title = match self.active_sidebar_pane {
            0 => "Requests",
            1 => "Sequences",
            _ => "Environments",
        };
        return div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.sidebar_searchbar(theme))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .child(title)
                    .text_sm()
                    .text_color(theme.muted_foreground),
            )
            .child(self.sidebar_tree(cx))
            .child(self.bottom_pane(theme, cx));
    }

    fn sidebar_searchbar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .min_h(px(40.))
            .max_h(px(40.))
            .pr_1()
            .gap_2()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(
                input::Input::new(&self.search_input_state)
                    .prefix(Icon::new(IconName::Search))
                    .appearance(false),
            )
            .child(button::Button::new("tests").icon(Icon::new(IconName::Plus)));
    }

    fn sidebar_tree(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let pane = self.active_sidebar_pane;
        let state = match pane {
            0 => &self.requests_tree_state,
            1 => &self.sequences_tree_state,
            _ => &self.env_tree_state,
        };
        let view = cx.weak_entity();

        return tree(state, move |ix, entry, selected, _, cx| {
            let mut item = match pane {
                0 => sidebar_requests::render_tree_row(ix, entry, selected, cx),
                1 => sidebar_sequences::render_tree_row(ix, entry, selected, cx),
                _ => sidebar_environments::render_tree_row(ix, entry, selected, cx),
            };

            if entry.is_folder() {
                return item;
            }

            let item_id = entry.item().id.clone();
            let item_label = entry.item().label.clone();
            let view = view.clone();

            item = item.on_click(move |_ev, window, app_cx| {
                if let Some(view) = view.upgrade() {
                    let kind = resources::ResourceType::from_id(&item_id)
                        .map(|rt| rt.to_tab_kind())
                        .unwrap_or(TabKind::Request);
                    view.update(app_cx, |this, cx| {
                        this.open_tab(
                            item_id.clone(),
                            item_label.clone(),
                            kind,
                            window,
                            cx,
                        );
                    });
                }
            });

            return item;
        })
        .flex_1()
        .min_h_0();
    }

    fn bottom_pane(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_sidebar_pane;
        return div()
            .flex()
            .gap_2()
            .p_2()
            .border_t(px(1.))
            .border_color(theme.border)
            .child(
                button::Button::new("switch-to-request-pill")
                    .label("requests")
                    .selected(active == 0)
                    .xsmall()
                    .on_click(cx.listener(move |this, _event, _window, _cx| {
                        this.active_sidebar_pane = 0;
                    })),
            )
            .child(
                button::Button::new("switch-to-sequences-pill")
                    .label("sequences")
                    .selected(active == 1)
                    .xsmall()
                    .on_click(cx.listener(move |this, _event, _window, _cx| {
                        this.active_sidebar_pane = 1;
                    })),
            )
            .child(
                button::Button::new("switch-to-environments-pill")
                    .label("envs")
                    .selected(active == 2)
                    .xsmall()
                    .on_click(cx.listener(move |this, _event, _window, _cx| {
                        this.active_sidebar_pane = 2;
                    })),
            );
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
                                    if let Some(pos) =
                                        this.find_tab_position(&tab_id)
                                    {
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
                    div().flex_1().child("Request panel not found").into_any_element()
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
                    div().flex_1().child("Environment panel not found").into_any_element()
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
}

impl Render for WorkspaceView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return app_wrapper::<Self>(window, cx)
            .child(components::title_bar::render(
                "Project name",
                env_popup::EnvPopup::new(self.env_popup_state.clone()),
                cx,
            ))
            .child(
                div().flex_1().min_h_0().child(
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
                                        .child(self.sidebar(&theme, cx)),
                                )
                                .child(resizable_panel().child(self.mainpanel(window, cx))),
                        ),
                ),
            );
    }
}

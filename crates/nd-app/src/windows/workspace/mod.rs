mod sidebar_requests;
mod sidebar_sequences;

use gpui::*;
use gpui_component::{
    button, input,
    resizable::{h_resizable, resizable_panel},
    tree::{tree, TreeState},
    ActiveTheme, Icon, IconName, Selectable, Sizable, Theme,
};

use crate::{
    ui::components::{self, env_popup, request::RequestPanel},
    windows::app_wrapper,
};

pub struct WorkspaceView {
    search_input_state: Entity<input::InputState>,
    requests_tree_state: Entity<TreeState>,
    sequences_tree_state: Entity<TreeState>,
    active_sidebar_pane: usize,
    env_popup_state: Entity<env_popup::EnvPopupState>,
    _rp: Entity<RequestPanel>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search requests..."));

        let env_popup_state =
            cx.new(|cx| env_popup::EnvPopupState::new(window, cx));

        let requests_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_requests::sample_tree_items()));
        let sequences_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_sequences::sample_sequence_items()));

        Self {
            search_input_state,
            requests_tree_state,
            sequences_tree_state,
            active_sidebar_pane: 0,
            env_popup_state,
            _rp: cx.new(|cx| RequestPanel::new(window, cx)),
        }
    }

    fn sidebar(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let title = if self.active_sidebar_pane == 0 {
            "Requests"
        } else {
            "Sequences"
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
            .min_h(px(50.))
            .max_h(px(50.))
            .px_2()
            .gap_2()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(input::Input::new(&self.search_input_state).prefix(Icon::new(IconName::Search)))
            .child(button::Button::new("tests").icon(Icon::new(IconName::Plus)));
    }

    fn sidebar_tree(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        let is_requests = self.active_sidebar_pane == 0;
        let state = if is_requests {
            &self.requests_tree_state
        } else {
            &self.sequences_tree_state
        };
        return tree(state, move |ix, entry, selected, _, cx| {
            if is_requests {
                sidebar_requests::render_tree_row(ix, entry, selected, cx)
            } else {
                sidebar_sequences::render_tree_row(ix, entry, selected, cx)
            }
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
            );
    }

    fn mainpanel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        return div().flex_1().child(self._rp.clone());
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

mod sidebar_request_lists;

use gpui::*;
use gpui_component::{
    button, input,
    tree::{tree, TreeState},
    ActiveTheme, Icon, IconName, Theme,
};

use crate::ui::components::request::RequestPanel;

pub struct WorkspaceView {
    search_input_state: Entity<input::InputState>,
    requests_tree_state: Entity<TreeState>,
    _rp: Entity<RequestPanel>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search requests..."));

        let requests_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_request_lists::sample_tree_items()));

        Self {
            search_input_state,
            requests_tree_state,
            _rp: cx.new(|cx| RequestPanel::new(window, cx)),
        }
    }

    fn sidebar(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .w_96()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .border_r(px(1.))
            .border_color(theme.border)
            .child(self.sidebar_searchbar(theme))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .child("Requests")
                    .text_sm()
                    .text_color(theme.muted_foreground),
            )
            .child(self.sidebar_request_tree(cx));
    }

    fn sidebar_searchbar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .p_3()
            .gap_2()
            .flex()
            .flex_row()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(input::Input::new(&self.search_input_state).prefix(Icon::new(IconName::Search)))
            .child(button::Button::new("tests").icon(Icon::new(IconName::Bot)));
    }

    fn sidebar_request_tree(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        return tree(&self.requests_tree_state, |ix, entry, selected, _, cx| {
            sidebar_request_lists::render_tree_row(ix, entry, selected, cx)
        })
        .flex_1()
        .min_h_0();
    }

    fn mainpanel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        return div().flex_1().child(self._rp.clone());
    }
}

impl Render for WorkspaceView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.sidebar(&theme, cx))
            .child(self.mainpanel(window, cx));
    }
}

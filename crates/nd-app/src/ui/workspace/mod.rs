mod sidebar_request_lists;

use gpui::*;
use gpui_component::{
    button::{self, ButtonVariants},
    h_flex, input,
    tree::{tree, TreeState},
    ActiveTheme, Icon, IconName, Sizable, StyledExt, Theme,
};

pub struct WorkspaceView {
    search_input_state: Entity<input::InputState>,
    requests_tree_state: Entity<TreeState>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search resources..."));

        let requests_tree_state =
            cx.new(|cx| TreeState::new(cx).items(sidebar_request_lists::sample_tree_items()));

        Self {
            search_input_state,
            requests_tree_state,
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
            .child(self.sidebar_request_tree(cx));
    }

    fn sidebar_searchbar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .p_3()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(input::Input::new(&self.search_input_state));
    }

    fn sidebar_request_tree(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        return tree(&self.requests_tree_state, |ix, entry, selected, _, cx| {
            sidebar_request_lists::render_tree_row(ix, entry, selected, cx)
        })
        .flex_1()
        .min_h_0();
    }

    fn mainpanel(&mut self) -> impl IntoElement {
        return div().flex_1().p_4().child("Main panel");
    }
}

impl Render for WorkspaceView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.sidebar(&theme, cx))
            .child(self.mainpanel());
    }
}

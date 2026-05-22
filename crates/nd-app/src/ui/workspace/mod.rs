mod sidebar_request_lists;

use gpui::*;
use gpui_component::{
    button, input,
    resizable::{h_resizable, resizable_panel},
    tree::{tree, TreeState},
    ActiveTheme, Icon, IconName, Sizable, Theme,
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
            .flex()
            .flex_col()
            .size_full()
            .child(self.sidebar_searchbar(theme))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .child("Requests")
                    .text_sm()
                    .text_color(theme.muted_foreground),
            )
            .child(self.sidebar_request_tree(cx))
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

    fn sidebar_request_tree(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        return tree(&self.requests_tree_state, |ix, entry, selected, _, cx| {
            sidebar_request_lists::render_tree_row(ix, entry, selected, cx)
        })
        .flex_1()
        .min_h_0();
    }

    fn bottom_pane(&mut self, theme: &Theme, _cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex()
            .gap_2()
            .p_2()
            .border_t(px(1.))
            .border_color(theme.border)
            .child(
                button::Button::new("switch-to-request-pill")
                    .label("requests")
                    .xsmall(),
            )
            .child(
                button::Button::new("switch-to-tests-pill")
                    .label("tests")
                    .xsmall(),
            );
    }

    fn mainpanel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        return div().flex_1().child(self._rp.clone());
    }
}

impl Render for WorkspaceView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return div()
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
            );
    }
}

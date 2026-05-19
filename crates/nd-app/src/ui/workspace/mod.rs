use gpui::*;
use gpui_component::{button, input, ActiveTheme, Icon, IconName, Sizable, Theme};

pub struct WorkspaceView {
    search_input_state: Entity<input::InputState>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search resources..."));

        return Self { search_input_state };
    }

    fn sidebar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .w_96()
            .flex()
            .gap_2()
            .flex_col()
            .border_r(px(1.))
            .border_color(theme.border)
            .child(self.sidebar_searchbar(theme))
            .child(
                self.sidebar_header(
                    "Requests".into(),
                    button::Button::new("add-request")
                        .small()
                        .icon(Icon::new(IconName::Plus).small()),
                ),
            )
            .child(self.sidebar_items(theme))
            .child(
                self.sidebar_header(
                    "Tests".into(),
                    button::Button::new("add-test")
                        .small()
                        .icon(Icon::new(IconName::Plus).small()),
                ),
            );
    }

    fn sidebar_header(&mut self, title: SharedString, right: impl IntoElement) -> impl IntoElement {
        return div()
            .px_3()
            .py_1()
            .flex()
            .flex_row()
            .gap_2()
            .items_center()
            .justify_between()
            .child(
                div()
                    .items_center()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(Icon::new(IconName::Network).small())
                    .child(title),
            )
            .child(right);
    }

    fn sidebar_searchbar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .p_3()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(input::Input::new(&self.search_input_state));
    }

    fn sidebar_items(&mut self, _theme: &Theme) -> impl Element {
        return div().p_3().gap_2().flex().flex_col();
    }

    fn mainpanel(&mut self) -> impl IntoElement {
        return div().child("Main panel");
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
            .child(self.sidebar(&theme))
            .child(self.mainpanel());
    }
}

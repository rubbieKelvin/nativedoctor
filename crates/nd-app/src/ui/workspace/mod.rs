use gpui::*;
use gpui_component::{input, ActiveTheme, Theme};

mod sidebar_lists;

// TODO: implement well
struct SidebarItem {
    method: String,
    label: String,
}

pub struct WorkspaceView {
    search_input_state: Entity<input::InputState>,
}

impl WorkspaceView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search resources..."));

        cx.subscribe(
            &search_input_state,
            move |_this, _entity, ev: &input::InputEvent, cx| {
                if matches!(ev, input::InputEvent::Change) {
                    cx.notify();
                }
            },
        )
        .detach();

        return Self { search_input_state };
    }

    fn sidebar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .w_112()
            .flex()
            .flex_col()
            .gap_2()
            .border_r(px(1.))
            .border_color(theme.border)
            .child(self.sidebar_searchbar(theme))
            .child(self.sidebar_items(
                theme,
                vec![
                    SidebarItem {
                        label: "Request One".into(),
                        method: "GET".into(),
                    },
                    SidebarItem {
                        label: "Request two".into(),
                        method: "POST".into(),
                    },
                ],
            ));
    }

    fn sidebar_searchbar(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .p_3()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(input::Input::new(&self.search_input_state));
    }

    fn sidebar_items(&mut self, theme: &Theme, items: Vec<SidebarItem>) -> impl Element {
        return div().p_3().gap_2().flex().flex_col().children(
            items
                .iter()
                .map(|item| {
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(
                            div()
                                .bg(theme.secondary)
                                .p_1()
                                .rounded_md()
                                .text_xs()
                                .child(item.method.clone()),
                        )
                        .child(item.label.clone())
                })
                .collect::<Vec<Div>>(),
        );
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

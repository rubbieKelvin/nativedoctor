use gpui::*;
use gpui_component::{
    button::{self, ButtonVariants},
    input,
    ActiveTheme, Icon, IconName, Root, TitleBar,
};

use crate::windows::{app_wrapper, workspace};

pub struct ProjectView {
    search_input_state: Entity<input::InputState>,
}

impl ProjectView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search projects..."));

        return Self {
            search_input_state,
        };
    }

    fn open_workspace(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(
                size(px(1280.), px(800.)),
                &*cx,
            )),
            titlebar: Some(TitleBar::title_bar_options()),
            focus: true,
            ..Default::default()
        };

        cx.spawn(|_, cx: &mut AsyncApp| {
            let cx = cx.clone();
            async move {
                cx.open_window(options, |window, cx: &mut App| {
                    let view = cx.new(|cx| workspace::WorkspaceView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .unwrap();
            }
        })
        .detach();
    }

    fn render_main_area(&self, theme: &gpui_component::Theme, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child("Nativedoctor"),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .child(
                        button::Button::new("create-project-btn")
                            .label("Create Project")
                            .with_variant(button::ButtonVariant::Primary)
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.open_workspace(window, cx);
                            })),
                    )
                    .child(
                        button::Button::new("open-project-btn")
                            .label("Open Project")
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.open_workspace(window, cx);
                            })),
                    ),
            );
    }

    fn render_recent_sidebar(&self, theme: &gpui_component::Theme) -> impl IntoElement {
        return div()
            .w(px(400.))
            .flex()
            .flex_col()
            .border_l(px(1.))
            .border_color(theme.border)
            .child(
                div()
                    .p_2()
                    .border_b(px(1.))
                    .border_color(theme.border)
                    .child(input::Input::new(&self.search_input_state).prefix(
                        Icon::new(IconName::Search),
                    ))
                    .child(
                        div()
                            .pt_2()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.foreground)
                            .child("Recent"),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child("No recent projects"),
                    ),
            );
    }
}

impl Render for ProjectView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return app_wrapper::<Self>(window, cx)
            .child(
                TitleBar::new().child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .size_full()
                        .text_sm()
                        .child(
                            div()
                                .text_color(theme.foreground)
                                .child("Nativedoctor"),
                        ),
                ),
            )
            .child(
                div().flex_1().min_h_0().child(
                    div()
                        .size_full()
                        .flex()
                        .flex_row()
                        .bg(theme.background)
                        .text_color(theme.foreground)
                        .child(self.render_main_area(&theme, cx))
                        .child(self.render_recent_sidebar(&theme)),
                ),
            );
    }
}

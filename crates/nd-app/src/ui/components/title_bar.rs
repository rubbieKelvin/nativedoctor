use gpui::*;
use gpui_component::{ActiveTheme, TitleBar};

pub fn render(
    project_name: impl Into<SharedString>,
    env_popup: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    let project_name = project_name.into();
    let theme = cx.theme();

    return TitleBar::new().child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .size_full()
            .gap_2()
            .text_sm()
            .child(div().text_color(theme.foreground).child(project_name))
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child("・")
                    .opacity(0.8),
            )
            .child(env_popup),
    );
}

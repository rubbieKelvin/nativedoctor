use gpui::*;
use gpui_component::{ActiveTheme, TitleBar};

pub fn render(
    project_popup: impl IntoElement,
    env_popup: impl IntoElement,
    cx: &App,
) -> impl IntoElement {
    let theme = cx.theme();

    return TitleBar::new().child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .size_full()
            .gap_2()
            .text_sm()
            .child(project_popup)
            .child(
                div()
                    .text_color(theme.muted_foreground)
                    .child("・")
                    .opacity(0.8),
            )
            .child(env_popup),
    );
}

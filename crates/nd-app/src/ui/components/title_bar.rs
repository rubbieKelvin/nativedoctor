use gpui::*;
use gpui_component::{ActiveTheme, Icon, IconName, Sizable, TitleBar};

pub fn render(
    project_name: impl Into<SharedString>,
    env: impl Into<SharedString>,
    cx: &App,
) -> impl IntoElement {
    let project_name = project_name.into();
    let env = env.into();
    let theme = cx.theme();

    return TitleBar::new().child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .text_sm()
            .child(div().text_color(theme.foreground).child(project_name))
            .child(div().text_color(theme.muted_foreground).child("/"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .id("title-bar-env")
                    .items_center()
                    .gap_1()
                    .text_color(theme.muted_foreground)
                    .child(Icon::new(IconName::ChevronsUpDown).xsmall())
                    .child(env),
            ),
    );
}

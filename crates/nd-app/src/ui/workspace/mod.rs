use gpui::*;
use gpui_component::{ActiveTheme as _, Theme};

pub fn workspace_view<T: 'static>(_window: &mut Window, cx: &mut Context<T>) -> impl IntoElement {
    let theme = cx.theme().clone();

    div()
        .flex()
        .flex_row()
        .size_full()
        .bg(theme.background)
        .text_color(theme.foreground)
        .child(sidebar(&theme))
}

fn sidebar(theme: &Theme) -> impl IntoElement {
    return div();
}

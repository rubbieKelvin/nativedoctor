use gpui::*;
use gpui_component::Root;

pub mod project;
pub mod settings;
pub mod workspace;

pub fn app_wrapper<T>(window: &mut Window, cx: &mut Context<T>) -> Div {
    return div()
        .flex()
        .flex_col()
        .size_full()
        .children(Root::render_dialog_layer(window, cx))
        .children(Root::render_sheet_layer(window, cx))
        .children(Root::render_notification_layer(window, cx));
}

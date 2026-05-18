use gpui::*;
use gpui_component::*;

use crate::ui;

pub struct ND;

impl Render for ND {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .size_full()
            .child(ui::workspace::workspace_view(window, cx))
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
            .children(Root::render_notification_layer(window, cx));
    }
}

pub fn setup() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        crate::theme::init(cx);

        // close app on last window close
        cx.on_window_closed(|app, _window| {
            if app.windows().len() == 0 {
                app.quit();
            }
        })
        .detach();

        // spawn window
        spawn_window(cx);
    });
}

pub fn window_options(cx: &App) -> WindowOptions {
    let bounds = WindowBounds::centered(size(px(1280.), px(800.)), cx);
    return WindowOptions {
        window_bounds: Some(bounds),
        ..Default::default()
    };
}

pub fn spawn_window(cx: &mut App) {
    let options = window_options(cx);

    cx.spawn(async move |cx| {
        cx.open_window(options, |window, cx| {
            let view = cx.new(|_| ND);
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
    })
    .detach();
}

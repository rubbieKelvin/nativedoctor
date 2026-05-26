use gpui::*;
use gpui_component::{Root, TitleBar};

use crate::windows::workspace;

pub fn setup() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);
        crate::theme::init(cx);

        // bring the window to the front
        cx.activate(true);

        // close app on last window close
        cx.on_window_closed(|app, _window| {
            if app.windows().len() == 0 {
                app.quit();
            }
        })
        .detach();

        spawn_workspace_window(cx);
    });
}

pub fn window_options(cx: &App) -> WindowOptions {
    let bounds = WindowBounds::centered(size(px(1280.), px(800.)), cx);
    return WindowOptions {
        window_bounds: Some(bounds),
        titlebar: Some(TitleBar::title_bar_options()),
        focus: true,
        ..Default::default()
    };
}

pub fn spawn_workspace_window(cx: &mut App) {
    let options = window_options(cx);

    cx.spawn(async move |cx| {
        cx.open_window(options, |window, cx| {
            let view = cx.new(|cx| workspace::WorkspaceView::new(window, cx));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
    })
    .detach();
}

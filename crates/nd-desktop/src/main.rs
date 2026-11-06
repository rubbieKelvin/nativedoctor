mod model;
mod runtime;
mod workspace;

use std::path::PathBuf;

use gpui_kit::component::TitleBar;
use gpui_kit::*;

use workspace::Workspace;

fn main() {
    let start = std::env::args_os().nth(1).map(PathBuf::from);
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx| {
            gpui_kit::init(cx);
            runtime::init(cx);
            cx.activate(true);
            cx.on_window_closed(|cx, _| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            let mut options = TitleBar::window_options();
            options.window_bounds = Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1200.), px(780.)),
                cx,
            )));
            gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| Workspace::new(start, window, cx))
            })
            .expect("failed to open window");
        });
}

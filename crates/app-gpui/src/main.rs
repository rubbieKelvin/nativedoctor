// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{
    App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, size,
};

mod constants;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        return div();
    }
}

fn main() {
    let app = Application::new();

    app.run(|cx: &mut App| {
        // Close the application once all windows are closed
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // Open window
        cx.open_window(
            WindowOptions {
                app_id: Some(constants::APP_ID.to_string()),
                titlebar: Some(TitlebarOptions {
                    title: Some(constants::APP_NAME.into()),
                    appears_transparent: true,
                    traffic_light_position: Default::default(),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(500.), px(500.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HelloWorld {}),
        )
        .unwrap();
    });
}

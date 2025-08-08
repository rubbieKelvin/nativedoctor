use gpui::{App, Application};

use crate::app::NativeDoctor;

mod app;
mod components;
mod menu;
mod states;
mod views;
mod runtime;

fn main() {
    env_logger::init();
    Application::new().run(|cx: &mut App| {
        // initialise menu bar
        menu::init(cx);

        // Close the application once all windows are closed
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        // Open window
        NativeDoctor::new(cx).unwrap();
        cx.activate(true);
    });
}

use gpui::{App, AppContext, Application};

mod app;
mod menu;

fn main() {
    let app = Application::new();

    app.run(|cx: &mut App| {
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
        cx.open_window(app::NativeDoctorApp::new_window_option(cx), |_, cx| {
            cx.new(|_| app::NativeDoctorApp::new())
        })
        .unwrap();

        cx.activate(true);
    });
}

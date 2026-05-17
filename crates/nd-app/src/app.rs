use gpui::*;
use gpui_component::{button::*, *};

pub struct ND;

impl Render for ND {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        return div();
    }
}

pub fn setup() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        // close app on last window close
        cx.on_window_closed(|app, _window| {
            if app.windows().len() == 0 {
                app.quit();
            }
        })
        .detach();

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| ND);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}

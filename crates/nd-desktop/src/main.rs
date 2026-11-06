mod model;

use gpui_kit::component::*;
use gpui_kit::*;

struct Workspace;

impl Render for Workspace {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        return div()
            .v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .child("NativeDoctor");
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| Workspace))
                .expect("failed to open window");
        });
}

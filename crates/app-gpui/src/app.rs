use gpui::{App, Bounds, Render, TitlebarOptions, WindowBounds, WindowOptions, div, px, size};
use nd_core::constants;

pub struct NativeDoctorApp;

impl NativeDoctorApp {
    pub fn new() -> Self {
        return NativeDoctorApp {};
    }

    pub fn new_window_option(cx: &App) -> WindowOptions {
        return WindowOptions {
            app_id: Some(constants::APPLICATION_ID.to_string()),
            titlebar: Some(TitlebarOptions {
                title: Some(constants::APPLICATION_NAME.into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(500.), px(500.0)),
                cx,
            ))),
            ..Default::default()
        };
    }
}

impl Render for NativeDoctorApp {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        return div();
    }
}

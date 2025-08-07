use gpui::{
    App, AppContext, Bounds, Context, Entity, ParentElement, Render, Size, Styled, TitlebarOptions,
    Window, WindowBounds, WindowHandle, WindowOptions, div, px, rgb, size,
};
use nd_core::constants;

use crate::{states::ApplicationState, views::ViewManager};

pub struct NativeDoctor {
    view_manager: ViewManager,
    state: Entity<ApplicationState>,
}

impl NativeDoctor {
    pub fn new(cx: &mut App) -> anyhow::Result<WindowHandle<Self>> {
        let bounds = WindowBounds::Windowed(Bounds::centered(None, size(px(1200.), px(800.0)), cx));
        let option = WindowOptions {
            app_id: Some(constants::APPLICATION_ID.to_string()),
            titlebar: Some(TitlebarOptions {
                title: Some(constants::APPLICATION_NAME.into()),
                appears_transparent: true,
                ..Default::default()
            }),
            window_bounds: Some(bounds),
            window_min_size: Some(Size::new(800.0.into(), 700.0.into())),
            ..Default::default()
        };

        return cx.open_window(option, |window, cx| {
            cx.new(|cx| NativeDoctor::new_window(window, cx))
        });
    }

    fn new_window(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        return NativeDoctor {
            state: cx.new(|cx| ApplicationState::new(cx)),
            view_manager: ViewManager::new(),
        };
    }
}

impl Render for NativeDoctor {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        return div()
            .size_full()
            .bg(rgb(0x191919))
            .child(self.view_manager.render(window, cx));
    }
}

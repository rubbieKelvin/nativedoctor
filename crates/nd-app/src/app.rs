//! Top-level GPUI application. Owns the root [`AppState`] entity and
//! switches between the landing page and workspace based on state.

use std::rc::Rc;

use gpui::*;

use crate::state::AppState;

pub struct NativeDoctorApp;

impl NativeDoctorApp {
    pub fn run() {
        Application::with_platform(Rc::new(gpui_macos::MacPlatform::new(false))).run(
            |cx: &mut App| {
                let state = cx.new(|_cx| AppState::new());

                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                            point(px(100.), px(60.)),
                            size(px(1200.), px(800.)),
                        ))),
                        titlebar: Some(TitlebarOptions {
                            title: Some("NativeDoctor".into()),
                            appears_transparent: false,
                            traffic_light_position: None,
                        }),
                        ..Default::default()
                    },
                    |_window: &mut Window, cx: &mut App| cx.new(|cx| RootView::new(cx, state)),
                )
                .unwrap();
            },
        );
    }
}

pub struct RootView {
    state: Entity<AppState>,
    _subscription: Subscription,
}

impl RootView {
    pub fn new(cx: &mut Context<Self>, state: Entity<AppState>) -> Self {
        let _subscription = cx.observe(&state, |_this, _model, cx| {
            cx.notify();
        });

        Self {
            state,
            _subscription,
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = {
            let state = self.state.read(cx);
            state.current_view.clone()
        };

        match view {
            crate::state::View::Landing => {
                crate::pages::landing::render_landing(_window, cx, self.state.clone())
                    .into_any_element()
            }
            crate::state::View::Workspace => {
                crate::pages::workspace::render_workspace(_window, cx, self.state.clone())
                    .into_any_element()
            }
        }
    }
}

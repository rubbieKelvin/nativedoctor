//! Top-level GPUI application. Owns the root [`crate::state::AppState`] entity and
//! switches between the landing page and workspace based on routing state.

use std::ops::DerefMut;

use gpui::*;
use gpui_component::Root;

use crate::state::AppState;

pub struct NativeDoctorApp;

impl NativeDoctorApp {
    /// Initialise gpui-component, tune the toolkit theme, open the flagship window rooted in [`Root`].
    pub fn run() {
        gpui_platform::application().run(|cx| {
            gpui_component::init(cx);
            crate::theme::apply_native_doctor_theme(cx);

            let shell_state = cx.new(|_: &mut Context<AppState>| AppState::new());

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(100.), px(60.)),
                        size(px(1280.), px(820.)),
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("NativeDoctor".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    ..Default::default()
                },
                move |window: &mut Window, cx: &mut App| {
                    let router =
                        cx.new(|shell_cx| ShellView::new(shell_cx, shell_state.clone()));
                    cx.new(|root_ctx| Root::new(router, window, root_ctx))
                },
            )
            .expect("Failed to spawn NativeDoctor window");
        });
    }
}

/// High-level routing shell that observes [`AppState`] so landing ↔ workspace swaps stay instantaneous.
pub struct ShellView {
    state: Entity<AppState>,
    _subscription: Subscription,
}

impl ShellView {
    /// Wire cross-entity observers so datastore mutations repaint this host view.
    pub fn new(cx: &mut Context<Self>, state: Entity<AppState>) -> Self {
        let observer_state = state.clone();
        let _subscription = cx.observe(&observer_state, |_shell, _, shell_cx| {
            shell_cx.notify();
        });

        Self {
            state,
            _subscription,
        }
    }
}

impl Render for ShellView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let route = self.state.read(cx).current_view.clone();

        match route {
            crate::state::View::Landing => {
                crate::pages::landing::render_landing(window, cx.deref_mut(), self.state.clone())
                    .into_any_element()
            }
            crate::state::View::Workspace => {
                crate::pages::workspace::render_workspace(window, cx.deref_mut(), self.state.clone())
                    .into_any_element()
            }
        }
    }
}

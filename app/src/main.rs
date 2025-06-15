use dioxus::{desktop::wry::dpi::Size, prelude::*};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

// use tracing::Level;
use views::{dashboard::DashboardView, empty::EmptyPage};

use states::{ApplicationState, ProjectContentLoadingStatus};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

mod components;
mod states;
mod views;

fn main() {
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let mut window_builder = WindowBuilder::new()
        .with_inner_size(Size::Logical(LogicalSize::new(1200.0, 800.0)))
        .with_transparent(true)
        .with_resizable(true);

    #[cfg(debug_assertions)]
    {
        window_builder = window_builder.with_always_on_top(true);
    }

    #[cfg(target_os = "macos")]
    {
        use dioxus_desktop::tao::platform::macos::WindowBuilderExtMacOS;

        window_builder = window_builder
            .with_titlebar_transparent(true)
            .with_title_hidden(true)
            .with_fullsize_content_view(true)
            .with_titlebar_buttons_hidden(false);
    }

    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_builder))
        .launch(App);

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // State
    let state = ApplicationState::provide();
    let project = state.project.read();

    // Ui element
    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        match *project {
            ProjectContentLoadingStatus::Loaded(_) => {
                rsx! {
                    DashboardView {}
                }
            }
            _ => {
                rsx! {
                    EmptyPage {}
                }
            }
        }
    };
}

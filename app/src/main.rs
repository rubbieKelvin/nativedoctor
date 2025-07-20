use dioxus::prelude::*;
use crate::session::Session;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

mod components;
mod session;
mod views;

fn main() {
    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::wry::dpi::Size;
        use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

        let mut window_builder = WindowBuilder::new()
            .with_inner_size(Size::Logical(LogicalSize::new(1200.0, 800.0)))
            .with_resizable(true)
            .with_focused(true)
            .with_visible(true);

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
                .with_fullsize_content_view(true);
        }

        dioxus::LaunchBuilder::desktop()
            .with_cfg(Config::new().with_window(window_builder))
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[derive(Clone, PartialEq)]
pub(crate) enum PageScreen {
    StartScreen,
    ProjectScreen(Session),
}

#[component]
fn App() -> Element {
    // State
    let screen_state = use_context_provider(|| Signal::new(PageScreen::StartScreen));

    // Ui element
    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        match screen_state() {
            PageScreen::StartScreen => rsx!{
                views::start::StartScreenView {  }
            },
            PageScreen::ProjectScreen(session) => rsx!{
                views::project::ProjectView {
                    session
                }
            }
        }
    };
}

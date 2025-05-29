use dioxus::{desktop::wry::dpi::Size, prelude::*};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
// use dioxus_free_icons::{icons::ld_icons::LdPlus, Icon};
use ui::{request_panel, side_bar};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

mod appdata;
mod ui;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Rustle")
                    // .with_always_on_top(true) // Leave this for development
                    .with_inner_size(Size::Logical(LogicalSize::new(1200.0, 800.0)))
                    .with_resizable(true),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    appdata::provide_context();

    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex h-full flex-row",
            side_bar::SideBar{}
            request_panel::RequestPanel {  }
        }
    };
}

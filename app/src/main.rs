use dioxus::{desktop::wry::dpi::Size, prelude::*};
use dioxus_desktop::{
    tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder,
};
// use dioxus_free_icons::{icons::ld_icons::LdPlus, Icon};
use appdata::{prelude::provide_context, requests, tabs};
use ui::{side_bar, work_view};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

mod appdata;
mod ui;

fn main() {
    let mut window_builder = WindowBuilder::new()
        .with_title("Rustle")
        .with_inner_size(Size::Logical(LogicalSize::new(1200.0, 800.0)))
        .with_transparent(true)
        .with_resizable(true);

    #[cfg(target_os = "macos")]
    {
        window_builder = window_builder
            .with_titlebar_transparent(true)
            .with_title_hidden(true)
            .with_fullsize_content_view(true)
            .with_titlebar_buttons_hidden(false);
    }

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window_builder))
        .launch(App);
}

#[component]
fn App() -> Element {
    provide_context();
    tabs::TabItemManager::provide();
    requests::RequestManager::provide();

    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        if cfg!(target_os = "macos") {
            document::Style {
                r#"
                html {{
                    padding: 2px;
                    border: 1px solid #353535;
                    border-radius: 8px;
                }}
                "#
            }
        }

        div {
            class: "flex h-full flex-row",
            side_bar::SideBar{}
            work_view::WorkPanel {}
        }
    };
}

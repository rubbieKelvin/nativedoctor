use dioxus::prelude::*;

mod session;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus::desktop::wry::dpi::Size;
        use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

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
            use dioxus::desktop::tao::platform::macos::WindowBuilderExtMacOS;

            window_builder = window_builder
                .with_titlebar_transparent(true)
                .with_title_hidden(true)
                .with_fullsize_content_view(true);
        }

        dioxus::LaunchBuilder::desktop()
            .with_cfg(Config::new().with_window(window_builder))
            .launch(App);
    }

    #[cfg(target_arch = "wasm32")]
    dioxus::LaunchBuilder::web().launch(App);
}


#[component]
fn App() -> Element {
    // Ui element
    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    };
}

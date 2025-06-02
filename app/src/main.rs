use appdata::{prelude::provide_context, project, requests, tabs};
use dioxus::{desktop::wry::dpi::Size, prelude::*};
use dioxus_desktop::{
    tao::platform::macos::WindowBuilderExtMacOS, Config, LogicalSize, WindowBuilder,
};
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
    project::ProjectManager::provide();
    requests::RequestManager::provide();

    // TODO: this is a hack to get the window to have a border on macos
    // but it's not working, so i'm not using it for now
    let macos_border = if cfg!(target_os = "macos") {
        // rsx! {
        //     document::Style {
        //         "
        //         html {{
        //             border-radius: 8px;
        //             box-sizing: border-box;

        //         }}
        //         body {{
        //             padding: 1px;
        //             border-radius: 8px;
        //             border: 1px solid #454445;
        //             overflow: clip;
        //             box-sizing: border-box;
        //         }}
        //         "
        //     }
        // }
        rsx!{}
    } else {
        rsx! {}
    };

    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        {macos_border}

        div {
            class: "flex h-full flex-row",
            oncontextmenu: move |e| {
                e.prevent_default();
            },
            side_bar::SideBar{}
            work_view::WorkPanel {}
        }
    };
}

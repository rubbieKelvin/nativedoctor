use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.output.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    return rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "bg-red-500 p-5",
            style: "height: 100%; display: flex;",
            div {
                style: "height: 100%;",
                "Smoke"
            }
            div {
                style: "height: 100%;",
                "Stack"
            }
        }
    };
}

// #[component]
// fn OpenProject() -> Element {
//     return rsx! {
//         div {
//             button {
//                 "Create project"
//             }
//             button {
//                 "Open project"
//             }
//         }
//     };
// }

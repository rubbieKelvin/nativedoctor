use dioxus::prelude::*;

mod button;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_JS: Asset = asset!("/assets/tailwind.js");

#[component]
fn App() -> Element {
    return rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: TAILWIND_JS }
        div { class: "p-4",
            h1 { "Preview" }

            div { class: "flex flex-col gap-2",
                for size in button::ButtonSizeVariant::all() {
                    div {
                        key: "{size.to_string()}", 
                        class: "flex gap-2",
                        p {
                            class: "text-sm text-gray-400",
                            "{size.to_string()}"
                        }
                        for style in button::ButtonStyleVariant::all() {
                            button::Button {
                                key: "{style.to_string()}",
                                style: style.clone(),
                                size: size.clone(),

                                if *size != button::ButtonSizeVariant::Icon {
                                    "{style.to_string()}"
                                }else{
                                    "O"
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}

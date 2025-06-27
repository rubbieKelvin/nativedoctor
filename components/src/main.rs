use dioxus::prelude::*;

mod button;
mod label;

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
        div { class: "flex gap-4 flex-col p-4",
            h1 { class: "mb-4","Preview" }

            // buttons
            div { class: "flex flex-col gap-2",
                h1 {
                    class: "text-3xl",
                    "Buttons"
                }
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

            // Labels
            div {
                class: "flex gap-2 flex-col",

                h1 {
                    "Label"
                }

                div {
                    class: "flex gap-4",

                    for style in label::LabelStyleVariant::all() {
                        div {
                            for size in label::LabelSizeVariant::all(){
                                label::Label{
                                    size: size.clone(),
                                    style: style.clone(),
                                    "{size.to_string()}"
                                }
                            }
                        }
                    }
                }
            }
            
            // 
        }
    };
}

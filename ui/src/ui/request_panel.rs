use dioxus::prelude::*;

#[component]
pub fn RequestPanel() -> Element {
    return rsx! {
        div {
            class: "h-full flex-grow",
            input {  placeholder: "METHOD"}
            input {  placeholder: "Enter url"}
        }
    };
}

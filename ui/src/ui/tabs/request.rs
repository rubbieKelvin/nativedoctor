use crate::appdata::prelude::RequestItem;
use dioxus::prelude::*;

#[component]
pub fn RequestPage(request: RequestItem) -> Element {
    return rsx! {
        div {
            class: "flex flex-col h-full",
        }
    };
}

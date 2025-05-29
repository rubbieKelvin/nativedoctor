use super::tabs::{page, tabs};
use dioxus::prelude::*;

#[component]
pub fn WorkPanel() -> Element {
    return rsx! {
        div {
            class: "flex flex-col h-full flex-grow",

            // tab listings
            tabs::TabItemManagerUi {  }

            // tab content
            page::TabPage {  }
        }
    };
}

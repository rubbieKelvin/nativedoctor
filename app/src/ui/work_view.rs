use super::tabs::{page, tabs};
use dioxus::prelude::*;
use rustle_ui_components::wm_drag_area::WmDragArea;

#[component]
pub fn WorkPanel() -> Element {
    return rsx! {
        div {
            class: "flex flex-col h-full flex-grow",

            // tab listings
            tabs::TabItemManagerUi {
                WmDragArea {
                    class: "flex-grow",
                }
            }

            // tab content
            page::TabPage {  }
        }
    };
}

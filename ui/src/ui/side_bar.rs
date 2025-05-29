use crate::ui::{
    enviroment_selector,
    toggle_bar::{env_toggle_bar, request_toggle_bar, sequence_toggle_bar},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div {
            class: "h-full flex flex-col border-r w-[22%]",

            // Environment selector
            div {
                class: "p-2 border-b flex gap-2",
                enviroment_selector::EnviromentSelector{}
                button {
                    class: "p-1 border rounded-md",
                    title: "Edit environment",
                    Icon {
                        icon: ld_icons::LdSquarePen,
                        width: 16,
                        height: 16,
                    }
                }
            }

            // Item toggles
            div {
                class: "flex-grow h-0 overflow-y-auto",
                // Request header
                request_toggle_bar::RequestToggleBar {  }
                
                // Call sequence section
                sequence_toggle_bar::SequenceToggleBar {  }
                
                // Environment variables
                env_toggle_bar::EnvToggleBar{}
            }

            // Settings and extra menus
            div {
                class: "p-2 border-t",
                button {
                    class: "flex items-center gap-2 hover:bg-item-hover-bg rounded-md px-2 py-1 w-full",
                    Icon {
                        icon: ld_icons::LdSettings,
                        width: 16,
                        height: 16,
                    }
                    p {
                        "Settings"
                    }
                }
            }
        }
    }
}

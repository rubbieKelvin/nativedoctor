use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_solid_icons, Icon};

#[component]
pub fn WmProjectButton() -> Element {
    rsx! {
        button {
            class: "flex items-center gap-2 px-1 rounded hover:bg-item-hover-bg/70 truncate text-nowrap",
            div {
                Icon {
                    icon: hi_solid_icons::HiFolder,
                    width: 16,
                    height: 16,
                    class: "text-accent"
                }
            }
            p {
                class: "text-sm",
                "Project name"
            }
        }
    }
}

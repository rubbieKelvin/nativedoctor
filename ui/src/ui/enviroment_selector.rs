use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn EnviromentSelector() -> Element {
    rsx! {
        div {
            class: "border border-gray-200 rounded-md p-2 flex items-center justify-between",
            p {
                class: "text-xs",
                "Environment"
            }
            Icon {
                icon: ld_icons::LdChevronDown,
                width: 16,
                height: 16,
            }
        }
    }
}

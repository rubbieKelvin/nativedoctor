use super::toggle_bar;
use crate::appdata;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn SequenceToggleBar() -> Element {
    let sequences = use_context::<Signal<Vec<appdata::SequenceItem>>>();

    return rsx! {
        toggle_bar::ToggleBar {
            title: "Sequence".to_string(),
            add_button: Some(rsx! {
                button {
                    class: "hover:bg-item-hover-bg rounded-md",
                    Icon {
                        icon: ld_icons::LdPlus,
                        width: 16,
                        height: 16,
                    }
                }
            }),
            body: Some(rsx! {
                div {
                    for sequence in sequences() {
                        button {
                            class: "w-full flex items-center gap-2 pl-4 pr-2 py-0.5 hover:bg-item-hover-bg",
                            p {
                                class: " ",
                                "{sequence.name}"
                            }
                        }
                    }
                }
            })
        }
    };
}

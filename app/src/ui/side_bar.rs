use crate::{
    appdata::tabs::{TabItem, TabItemManager, TabType},
    ui::{
        enviroment_selector,
        toggle_bar::{env_toggle_bar, request_toggle_bar, sequence_toggle_bar},
    },
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn SideBar() -> Element {
    let mut tab_manager = use_context::<Signal<TabItemManager>>();
    // let mut open_env_edit_dialog = use_signal(|| false);

    rsx! {
        div {
            class: "h-full flex flex-col border-r w-[22%]",

            // Dialogs
            // env_edit_dialog::EnvEditDialog {
            //     open: open_env_edit_dialog,
            // }

            // Environment selector
            div {
                class: "py-1 px-2 border-b flex gap-2",
                enviroment_selector::EnviromentSelector{}

                // Edit environment button
                button {
                    class: "p-1 border rounded-md",
                    title: "Edit environment",
                    onclick: move |_| {
                        let tman = &mut tab_manager.write();
                        let tab = TabItem::new("Edit environment".to_string(), TabType::EditEnvironment, None);
                        tman.add_tab(tab);
                    },
                    Icon {
                        icon: ld_icons::LdSquarePen,
                        width: 14,
                        height: 14,
                    }
                }

                // Search button
                button {
                    class: "p-1 border rounded-md",
                    title: "Search",
                    onclick: move |_| {},
                    Icon {
                        icon: ld_icons::LdSearch,
                        width: 14,
                        height: 14,
                    }
                }
            }

            // Item toggles
            div {
                class: "flex-grow h-0 overflow-y-auto flex flex-col",
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

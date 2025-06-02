use crate::{
    appdata::{
        prelude::Environment,
        tabs::{TabItem, TabItemManager, TabType},
    },
    ui::{
        toggle_bar::{env_toggle_bar, request_toggle_bar, sequence_toggle_bar},
        wm_project_button::WmProjectButton,
    },
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};
use rustle_ui_components::{select::Select, wm_drag_area::WmDragArea};

#[component]
pub fn SideBar() -> Element {
    let environments = use_context::<Signal<Vec<Environment>>>();
    let mut tab_manager = TabItemManager::inject();

    let selected_environment = use_signal(|| Some(environments()[0].clone()));

    rsx! {
        div {
            class: "h-full flex flex-col border-r w-[22%] bg-bg-secondary",

            if cfg!(target_os = "macos") {
                WmDragArea {
                    class: "h-7 w-full pl-18 pr-2",

                    div {
                        class: "pt-1 flex gap-2 items-center w-full justify-between",
                        WmProjectButton {  }
                        p {
                            class: "text-sm text-text-secondary/50",
                            "v0.0.1"
                        }
                    }
                }
            }

            // Environment selector
            div {
                class: "py-1 px-2 flex gap-2",
                Select<Environment> {
                    items: environments(),
                    selected: selected_environment,
                    render_selected: |environment: &Environment| environment.name.clone(),
                    render_item: |environment: &Environment| rsx! { div { class: "px-2 py-0.1 text-text-secondary", "{environment.name}" } },
                    placeholder: "Select environment",
                    class: "px-2",
                    wrapper_class: "w-full border rounded-md hover:bg-item-hover-bg/50",
                    dropdown_class: "bg-bg-primary border",
                    item_class: "hover:bg-item-hover-bg",
                }


                // Edit environment button
                button {
                    class: "p-1 border rounded-md hover:bg-item-hover-bg",
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
                    class: "p-1 border rounded-md hover:bg-item-hover-bg",
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
                class: "p-2",
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

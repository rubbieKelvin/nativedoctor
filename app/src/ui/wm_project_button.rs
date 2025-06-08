use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_solid_icons, Icon};
use rfd::AsyncFileDialog;
use rustle_ui_components::popup::Popup;

use crate::appdata::project::ProjectManager;

#[component]
pub fn WmProjectButton() -> Element {
    let mut project_manager_signal = ProjectManager::inject();
    let project_manager = project_manager_signal();
    let mut open = use_signal(|| false);

    return rsx! {
        Popup {
            is_open: open,
            item: rsx!{
                button {
                    class: "flex items-center gap-2 px-1 rounded hover:bg-item-hover-bg/70 truncate text-nowrap",
                    // onclick: move |_| {
                    //     let folder_picker = AsyncFileDialog::new()
                    //         .set_title("Select Rustle Project File")
                    //         .add_filter("Rustle Project File", &["yaml", "yml"]);

                    //     spawn(async move {
                    //         if let Some(folder_path) = folder_picker.pick_file().await {
                    //             if let Some(path) = folder_path.path().to_str() {
                    //                 project_manager_signal.with_mut(|man| {
                    //                     man.open(path.to_string());
                    //                 })
                    //             }
                    //         }
                    //     });
                    // },
                    onclick: move |_| {
                        open.set(true);
                    },
                    if project_manager.current.is_some() {
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
                            { project_manager.current.unwrap().name }
                        }
                    }else{
                        div {

                            Icon {
                                icon: hi_solid_icons::HiFolderOpen,
                                width: 16,
                                height: 16,
                                class: "text-text-secondary/50"
                            }
                        }
                        p {
                            class: "text-sm",
                            "Select project"
                        }
                    }
                }
            },
            content: rsx!{SelectProjectPopupContent{}}
        }
    };
}

#[component]
fn SelectProjectPopupContent() -> Element {
    return rsx! {
        div{
            class: "bg-bg-primary p-8 h-full w-full rounded",
            p {
                class: "text-sm",
                "No project selected"
            }
        }
    };
}

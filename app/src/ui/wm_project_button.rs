use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_solid_icons, Icon};
use rfd::AsyncFileDialog;

use crate::appdata::project::ProjectManager;

#[component]
pub fn WmProjectButton() -> Element {
    let mut project_manager_signal = ProjectManager::inject();
    let project_manager = project_manager_signal();

    
    return rsx! {
        button {
            class: "flex items-center gap-2 px-1 rounded hover:bg-item-hover-bg/70 truncate text-nowrap",
            onclick: move |_| {
                let folder_picker = AsyncFileDialog::new();
                spawn(async move {
                    if let Some(folder_path) = folder_picker.pick_folder().await {
                        if let Some(path) = folder_path.path().to_str() {
                            project_manager_signal.with_mut(|man| {
                                man.open(path.to_string());
                            })
                        }
                    }
                });
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
    };
}

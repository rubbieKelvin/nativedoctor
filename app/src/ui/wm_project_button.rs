use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_solid_icons, Icon};

use crate::appdata::project::ProjectManager;

#[component]
pub fn WmProjectButton() -> Element {
    let project_manager_signal = ProjectManager::inject();
    let project_manager = project_manager_signal();

    return rsx! {
        button {
            class: "flex items-center gap-2 px-1 rounded hover:bg-item-hover-bg/70 truncate text-nowrap",
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
                    "Project name"
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

use dioxus::prelude::*;
use dioxus_free_icons::{icons::hi_outline_icons, Icon};
use rfd::AsyncFileDialog;

use crate::{
    actions::ProjectActions,
    components::WmDragArea,
    constants::{APP_NAME, FILE_EXTENSION},
    managers::ProjectStateManager,
};

#[component]
pub fn DashboardView() -> Element {
    return rsx! {
        div { class: "h-full", DashboardHeader {} }
    };
}

#[component]
pub fn DashboardHeader() -> Element {
    // let (project_state_signal, dispatch) = ProjectStateManager::inject();
    // let project_signal = project_state_signal();

    return rsx! {
        WmDragArea { class: "py-2 flex items-center",

            // env select
            div { class: "flex-grow pl-28 items-center gap-2 flex",
                // open button
                button {
                    class: "bg-gray-100 rounded px-2",
                    // onclick: {
                    //     let dispatch = dispatch.clone();
                    //     move |_| {
                    //         let mut dispatch = dispatch.clone();
                    //         let file_picker = AsyncFileDialog::new()
                    //             .set_title(format!("Select {} file", APP_NAME))
                    //             .add_filter("Project ", &[FILE_EXTENSION]);
                    //         spawn(async move {
                    //             if let Some(file_path) = file_picker.pick_file().await {
                    //                 let path = file_path.path();
                    //                 if path.is_file() {
                    //                     if let Some(path) = path.to_str().map(|s| s.to_string()) {
                    //                         dispatch(ProjectActions::LoadFile(path)).unwrap();
                    //                     }
                    //                 }
                    //             }
                    //         });
                    //     }
                    // },

                    // if let Some(path) = project_signal.file_name(false) {
                    //     span { "{path}" }
                    // } else {
                    //     span { "open" }
                    // }
                }


                // create button 
                button {
                    class: "bg-gray-100 rounded p-1",
                    // onclick: {
                    //     // TODO: this should create a new project
                    //     let dispatch = dispatch.clone();
                    //     move |_| {
                    //         let mut dispatch = dispatch.clone();
                    //         let file_picker = AsyncFileDialog::new().set_title("Select folder");
                    //         spawn(async move {
                    //             if let Some(folder_path) = file_picker.pick_folder().await {
                    //                 let path = folder_path.path();
                    //                 if path.is_file() {
                    //                     if let Some(path) = path.to_str().map(|s| s.to_string()) {
                    //                         dispatch(ProjectActions::LoadFile(path)).unwrap();
                    //                     }
                    //                 }
                    //             }
                    //         });
                    //     }
                    // },

                    Icon {
                        icon: hi_outline_icons::HiPlus,
                        width: 14,
                        height: 14,
                    }
                }
            }
        }
    };
}

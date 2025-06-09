use dioxus::prelude::*;
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
    let (project_state_signal, mut dispatch) = ProjectStateManager::inject();
    let project_signal = project_state_signal();

    return rsx! {
        WmDragArea { class: "py-2 flex items-center",

            // env select
            div { class: "flex-grow pl-28",
                button {
                    class: "bg-gray-100 rounded px-2",
                    onclick: move |_| {
                        let mut dispatch = dispatch.clone();
                        let file_picker = AsyncFileDialog::new()
                            .set_title(format!("Select {} file", APP_NAME))
                            .add_filter("Project ", &[FILE_EXTENSION]);

                        spawn(async move {
                            if let Some(file_path) = file_picker.pick_file().await {
                                let path = file_path.path();
                                if path.is_file() {
                                    if let Some(path) = path.to_str().map(|s| s.to_string()) {
                                        dispatch(ProjectActions::LoadFile(path)).unwrap();
                                    }
                                }
                            }
                        });
                    },

                    if let Some(path) = project_signal.file_name(false) {
                        span { "{path}" }
                    }else{
                        span { "open" }
                    }
                }
            }
        }
    };
}

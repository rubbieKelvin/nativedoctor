use dioxus::prelude::*;

use crate::states::ProjectState;

// Manages the current ui state of the requests list
#[component]
pub fn SideBar() -> Element {
    let project_state = ProjectState::inject();
    let requests = project_state.requests.read().clone();

    return match &*project_state.project.read() {
        Some(project) => rsx! {
            div { class: "w-[300px]",
                // name and version
                div {
                    span { "Name: {project.object.project.name}" }
                    br {}
                    span { "version: {project.object.project.get_version()}" }
                }

                // requests
                div {
                    h1 { "requests" }

                    button {
                        class: "bg-gray-300 p-2",
                        onclick: {
                            let mut project_state = project_state.clone();
                            move |_| {
                                project_state.add_new_request();
                            }
                        },
                        "Add requests"
                    }

                    div { class: "flex flex-col gap-1",
                        for request in requests {
                            button {
                                class: format!(
                                    "bg-gray-200 hover:bg-gray-400 {}",
                                    match &*project_state.selected_request.read() {
                                        Some(selected_request) => {
                                            if *selected_request == request.id { "" } else { "" }
                                        }
                                        None => "",
                                    },
                                ),
                                onclick: {
                                    let mut selected_request = project_state.selected_request.clone();
                                    move |_| {
                                        let mut writable_state = selected_request.write();
                                            *writable_state = Some(request.id.clone());
                                    }
                                },
                                {request.get_name().unwrap_or("Untitled Request".to_string())}
                            }
                        }
                    }
                }
            }
        },
        None => rsx! {
            div { "edge case" }
        },
    };
}

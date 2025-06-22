use dioxus::prelude::*;

use crate::states::ProjectState;

// Manages the current ui state of the requests list
#[component]
pub fn SideBar() -> Element {
    let project_state = ProjectState::inject();
    let open_request_signal = project_state.open_request.clone();
    let project_signal = project_state.project.clone();
    let requests = project_state.requests.read().clone();

    let project = project_signal();

    return rsx! {
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
                                "hover:bg-gray-400 {}",
                                match open_request_signal() {
                                    Some(open) => {
                                        if open.id == request.id { "bg-gray-400" } else { "bg-gray-200" }
                                    }
                                    None => "",
                                },
                            ),
                            onclick: {
                                let request = request.clone();
                                let mut open_request_signal = open_request_signal.clone();
                                move |_| {
                                    let mut writable_state = open_request_signal.write();
                                        *writable_state = Some(request.clone());
                                }
                            },
                            {request.name.clone()}
                        }
                    }
                }
            }
        }
    };
}

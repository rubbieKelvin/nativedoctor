use dioxus::prelude::*;
use nativedoctor_core::{fs::FileObject, schema::roots::{ProjectRootSchema, RequestRootSchema}};

#[component]
pub fn SideBar() -> Element {
    let project = use_context::<Signal<Option<FileObject<ProjectRootSchema>>>>();
    let requests = use_context::<Signal<Vec<FileObject<RequestRootSchema>>>>();

    return match project() {
        Some(project) => rsx! {
            div {
                class: "bg-accent w-[300px]",
                // name and version
                div {
                    span {
                        "Name: {project.object.project.name}"
                    }
                    br {  }
                    span {
                        "version: {project.object.project.get_version()}"
                    }
                }

                // requests
                div {
                    h1{
                        "requests"
                    }

                    for request in requests() {
                        button {
                            {request.get_name()}
                        }
                    }
                }
            },
        },
        None => rsx! {
            div {  "edge case" }
        },
    };
}

use crate::states::{ProjectState, WritableRequest};
use components_lib::{
    border::Border,
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    buttongroup::{ButtonGroup, GroupButton},
    label::{Label, LabelStyleVariant},
    pane::{Pane, PaneStyleVariant},
    select::Select,
};
use dioxus::prelude::*;

use dioxus_free_icons::{
    icons::ld_icons::{LdPencil, LdPlus},
    Icon,
};
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

use crate::components::WmDragArea;

#[component]
pub fn ProjectView(
    schema: FileObject<ProjectRootSchema>,
    requests: Vec<FileObject<RequestRootSchema>>,
) -> Element {
    let project_state = use_context_provider(|| ProjectState::new(schema, requests));
    let mut saving = use_signal(|| false);

    // load project in scope
    // {
    //     // clone upper scope
    //     let path = path.clone();
    //     let mut project = project_state.project.clone();

    //     use_effect(move || {
    //         tracing::info!("Loading project, {:?}", &path);

    //         let path = path.clone();

    //         // Asynchronously load up the project from file
    //         spawn(async move {
    //             let p = ProjectRootSchema::load(&path).await;
    //             match p {
    //                 Ok(p) => {
    //                     let mut project = project.write();
    //                     *project = Some(p);
    //                 }
    //                 // TODO: we need to let the user know an error showed up somehow.
    //                 Err(e) => tracing::error!("{e}"),
    //             };
    //         });
    //     })
    // };

    return rsx! {
        div { class: "h-full flex",
            WmDragArea { class: "h-8 w-full items-center fixed z-[9999]" }

            SideBar {}
            Pane { class: "flex-grow", style: PaneStyleVariant::Dark }
        }
    };
}

#[derive(PartialEq, Clone)]
enum SideBarList {
    Requests,
    Calls,
}

// Manages the current ui state of the requests list
#[component]
fn SideBar() -> Element {
    let project_state = ProjectState::inject();
    let open_request_signal = project_state.open_request.clone();
    let project_signal = project_state.project.clone();
    let requests = project_state.requests.read().clone();

    let project = project_signal();
    let mut current_list = use_signal(|| SideBarList::Requests);

    // this should be part of open project session
    let selected_env = use_signal(|| Some("Development".to_string()));

    return rsx! {
        Pane {
            style: PaneStyleVariant::Darker,
            border: Border::right(),
            class: "w-[300px] h-full flex flex-col gap-2",

            // name and version
            div { class: "pl-18 pt-1",
                Label { style: LabelStyleVariant::Ghost, "{project.object.project.name}" }
            }

            // env selector and buttons
            div { class: "flex px-2 pt-2 items-center gap-2",

                Select::<String> {
                    class: "w-full h-full",
                    wrapper_class: "flex-grow",
                    value: selected_env,
                    items: vec!["Development".into(), "Staging".into(), "Production".into()],
                }
                Button {
                    size: ButtonSizeVariant::Icon,
                    style: ButtonStyleVariant::Ghost,
                    Icon { width: 14, height: 14, icon: LdPencil }
                }
                Pane {
                    class: "h-full",
                    style: PaneStyleVariant::Transparent,
                    border: Border::right(),
                }
                Button {
                    size: ButtonSizeVariant::Icon,
                    style: ButtonStyleVariant::Ghost,
                    onclick: {
                        let mut project_state = project_state.clone();
                        move |_| {
                            project_state.add_new_request();
                        }
                    },
                    Icon { width: 14, height: 14, icon: LdPlus }
                }
            }

            // list tab
            div { class: "px-2",
                ButtonGroup {
                    class: "flex gap-2",
                    inactive_style: ButtonStyleVariant::Ghost,
                    GroupButton { class: "flex-[50%]",
                        onclick: move |_| {
                            current_list.set(SideBarList::Requests);
                        },
                        Label { "Requests" }
                    }
                    GroupButton { class: "flex-[50%]",
                        onclick: move |_| {
                            current_list.set(SideBarList::Calls);
                        },
                        Label { "Calls" }
                    }
                }
            }

            Pane {
                class: "w-full",
                style: PaneStyleVariant::Transparent,
                border: Border::bottom(),
            }

            // requests
            div {
                class: "flex-grow",

                if current_list() == SideBarList::Requests {
                    RequestList{}
                }else{
                    div {
                        "Calls"
                    }
                }
            }
        }
    };
}

#[component]
pub fn RequestList() -> Element {
    return rsx! {
        div {
            
        }
    };
}

#[component]
pub fn RequestPanel(request: WritableRequest) -> Element {
    let mut project_state = ProjectState::inject();
    let project = project_state.project.clone();

    let mut name = use_signal(|| String::new());
    // let request_memo = {
    //     let project_state = project_state.clone();
    //     use_memo(move || project_state.get_selected_request())
    // };

    // use_effect(move || {
    //     match request_memo() {
    //         Some(request) => {
    //             let n = request.get_name().unwrap_or_else(|| String::new());
    //             name.with_mut(|name_mut| {
    //                 *name_mut = n;
    //             });
    //         },
    //         None => {}
    //     };
    // });

    // let request_name = {
    //     let request = request_memo();
    //     use_memo(move || match &request {
    //         Some(r) => r.get_name().unwrap_or_else(|| String::new()),
    //         None => String::new(),
    //     })
    // };

    // use_effect(move || {
    //     let name = name();
    //     match request_memo() {
    //         Some(mut request_copy) => {
    //             request_copy.set_name(&name, &project().path);
    //             // project_state.update_request(request_copy);
    //         }
    //         None => {}
    //     };
    // });

    return rsx! {
        div { class: "flex-grow",
            "{request.id}"
            br {}
            span { "Name" }
            input {
                value: "{name}",
                placeholder: "Request name",
                oninput: move |e| {
                    let mut name = name.write();
                    *name = e.value().to_ascii_lowercase();
                },
                        // {
            //     let mut project_state = project_state.clone();

            //     move |e: Event<FormData>| {
            //         let value = e.value();
            //         let value = value.to_lowercase();
            //         let mut request = request_memo().unwrap();
            //         request.set_name(&value, &project().unwrap().path);
            //         project_state.update_request(request);
            //     }
            // },
            }
        }
    };
}

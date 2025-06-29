use crate::{session::Session, states::WritableRequest};
use components_lib::{
    border::Border,
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    buttongroup::{ButtonGroup, GroupButton},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    pane::{Pane, PaneStyleVariant},
    select::Select,
};
use dioxus::prelude::*;

use dioxus_free_icons::{
    icons::ld_icons::{LdEllipsisVertical, LdPencil, LdPlus},
    Icon,
};

use crate::components::WmDragArea;

#[component]
pub fn ProjectView(session: Session) -> Element {
    let mut saving = use_signal(|| false);
    let session = use_context_provider(|| Signal::new(session));

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
    // let project_state = ProjectState::inject();
    // let open_request_signal = project_state.open_request.clone();
    // let project_signal = project_state.project.clone();
    let mut session = use_context::<Signal<Session>>();

    let mut current_list = use_signal(|| SideBarList::Requests);
    let environments = use_memo(move || session().get_environments());

    // this should be part of open project session
    let selected_env = use_signal(|| {
        let environments = environments();
        if environments.len() > 0 {
            return Some(environments[0].clone());
        }
        return None;
    });

    // if seleted env changes, let's update session.current_env
    use_effect(move || {
        let mut session = session.write();
        let selected_env = selected_env();
        session.current_env = selected_env;
    });

    return rsx! {
        Pane {
            style: PaneStyleVariant::Darker,
            border: Border::right(),
            class: "w-[300px] h-full flex flex-col",

            // name and version
            div { class: "pl-18 pt-1",
                Label { style: LabelStyleVariant::Ghost, "{session().name}" }
            }

            // env selector and buttons
            div { class: "flex px-2 py-2 items-center gap-2",

                Select::<String> {
                    class: "w-full h-full",
                    wrapper_class: "flex-grow",
                    value: selected_env,
                    placeholder: "--No env--",
                    items: environments(),
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
                    onclick: move |_| {
                        let mut session = session.write();
                        session.new_empty_request();
                    },
                    Icon { width: 14, height: 14, icon: LdPlus }
                }
            }

            // list tab
            div { class: "px-2 mb-2",
                ButtonGroup {
                    class: "flex gap-2",
                    inactive_style: ButtonStyleVariant::Ghost,
                    GroupButton {
                        class: "flex-[50%]",
                        onclick: move |_| {
                            current_list.set(SideBarList::Requests);
                        },
                        Label { "Requests" }
                    }
                    GroupButton {
                        class: "flex-[50%]",
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
            div { class: "flex-grow h-0 overflow-y-auto",

                if current_list() == SideBarList::Requests {
                    RequestList { class: ""}
                } else {
                    div { "Calls" }
                }
            }
        }
    };
}

#[component]
pub fn RequestList(class: Option<String>) -> Element {
    let session = use_context::<Signal<Session>>();

    return rsx! {
        div {
            class,
            for request in session().requests {
                div {
                    class: "flex gap-2 px-2 items-center group/requestitem hover:bg-[#202020] py-1",
                    Label {
                        class: "lowercase",
                        size: LabelSizeVariant::Small,
                        "{request.method}"
                    }
                    Label {
                        class: "flex-grow",
                        "{request.name}"
                    }
                    Button {
                        class: "opacity-0 group-hover/requestitem:opacity-100",
                        style: ButtonStyleVariant::Transparent,
                        size: ButtonSizeVariant::Icon,
                        Icon{
                            width: 16,
                            height: 16,
                            class: "text-white",
                            icon: LdEllipsisVertical
                        }
                    }
                }
            }
        }
    };
}

#[component]
pub fn RequestPanel(request: WritableRequest) -> Element {
    // let mut name = use_signal(|| String::new());
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
        // div { class: "flex-grow",
        //     "{request.id}"
        //     br {}
        //     span { "Name" }
        //     input {
        //         value: "{name}",
        //         placeholder: "Request name",
        //         oninput: move |e| {
        //             let mut name = name.write();
        //             *name = e.value().to_ascii_lowercase();
        //         },
        //                 // {
        //     //     let mut project_state = project_state.clone();

        //     //     move |e: Event<FormData>| {
        //     //         let value = e.value();
        //     //         let value = value.to_lowercase();
        //     //         let mut request = request_memo().unwrap();
        //     //         request.set_name(&value, &project().unwrap().path);
        //     //         project_state.update_request(request);
        //     //     }
        //     // },
        //     }
        // }
    };
}

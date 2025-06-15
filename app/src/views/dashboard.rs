use crate::{
    components::WmDragArea,
    states::{ApplicationState, RequestLoadingStatus},
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum MainDashboardTab {
    Requests,
    Sequence,
}

#[component]
pub fn DashboardView() -> Element {
    let appstate = ApplicationState::inject();
    let tab = use_context_provider::<Signal<MainDashboardTab>>(|| {
        Signal::new(MainDashboardTab::Requests)
    });

    let project_info = appstate.current_project_title();

    return rsx! {
        div { class: "h-full flex flex-col",
            WmDragArea { class: "h-10 flex items-center pl-24",
                match project_info {
                    Some((filename, title)) => {
                        rsx! {
                            div {
                                class: "flex gap-1",
                                p { "{filename}" }
                                if let Some(title) = title {
                                    p {"-"}
                                    p {"{title}"}
                                }
                            }
                        }
                    }
                    None => {
                        rsx! {
                            p { "No open project" }
                        }
                    }
                }
            }

            div { class: "flex-grow flex",

                SideBar {}

                div { class: "flex-grow",
                    match tab() {
                        MainDashboardTab::Requests => rsx! {
                            RequestView {}
                        },
                        MainDashboardTab::Sequence => rsx! {
                            SequenceView {}
                        },
                    }
                }
            }
        }
    };
}

#[component]
fn SideBar() -> Element {
    let mut tab = use_context::<Signal<MainDashboardTab>>();

    return rsx! {
        div { class: "flex bg-gray flex-col p-4 gap-2",
            button {
                class: "p-2 rounded-md bg-gray-100 hover:bg-gray-300",
                onclick: move |_| {
                    *tab.write() = MainDashboardTab::Requests;
                },
                "Requests"
            }
            button {
                class: "p-2 rounded-md bg-gray-100 hover:bg-gray-300",
                onclick: move |_| {
                    *tab.write() = MainDashboardTab::Sequence;
                },
                "Sequence"
            }
        }
    };
}

#[component]
fn RequestListColumn() -> Element {
    let mut appstate = ApplicationState::inject();
    let requests = appstate.computed_requests();

    return rsx! {
        div { class: "border border-gray-300 mb-4 p-2",

            div { class: "flex gap-2",
                input { placeholder: "Search" }


                button {
                    class: "bg-gray-400",
                    onclick: move |_| {
                        appstate.add_new_request();
                    },
                    "create"
                }
            }

            div {
                for item in requests {
                    div { key: item.id, "{item.name}" }
                }
            }
        }
    };
}

#[component]
fn RequestPanelColumn() -> Element {
    return rsx! {
        div { class: "flex-grow", "Panel" }
    };
}

#[component]
fn RequestView() -> Element {
    return rsx! {
        div { class: "flex h-full",
            RequestListColumn {}
            RequestPanelColumn {}
        }
    };
}

#[component]
fn SequenceView() -> Element {
    return rsx! {
        div { "sequence" }
    };
}

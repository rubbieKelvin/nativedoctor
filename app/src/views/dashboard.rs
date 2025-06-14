use crate::components::WmDragArea;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
enum MainDashboardTab {
    Requests,
    Sequence,
}

#[component]
pub fn DashboardView() -> Element {
    let tab = use_context_provider::<Signal<MainDashboardTab>>(|| {
        Signal::new(MainDashboardTab::Requests)
    });

    return rsx! {
        div { class: "h-full flex flex-col",
            WmDragArea { class: "h-10 flex items-center bg-amber-400" }

            div {
                class: "flex-grow flex",

                SideBar{}

                div {
                    class: "bg-indigo-600 flex-grow",
                    match tab() {
                        MainDashboardTab::Requests => rsx!{RequestView {}},
                        MainDashboardTab::Sequence => rsx!{SequenceView {}}
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
        div {
            class: "flex bg-gray-200 flex-col p-4 gap-2",
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
    return rsx! {
        div {
            input {
                placeholder: "Search"
            }
        }
    };
}

#[component]
fn RequestPanelColumn() -> Element {
    return rsx! {
        div {
            class: "bg-lime-600 flex-grow",
            "Panel"
        }
    };
}

#[component]
fn RequestView() -> Element {
    return rsx! {
        div {
            class: "flex bg-fuchsia-800 h-full",
            RequestListColumn {  }
            RequestPanelColumn {  }
        }
    };
}

#[component]
fn SequenceView() -> Element {
    return rsx! {
        div {"sequence"}
    };
}

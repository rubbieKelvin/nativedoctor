use components_lib::{
    border::Border,
    button::{Button, ButtonStyleVariant},
    buttongroup::{ButtonGroup, ButtonGroupInner},
    label::Label,
    pane::Pane,
    select::Select,
    tabs::TabState,
    textfield::TextField,
};
use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::views::project::WorkspaceTab;

const HTTP_METHODS: [&'static str; 9] = [
    "GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "CONNECT", "OPTIONS", "TRACE",
];

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display)]
enum RequestInputTab {
    Params,
    Authorization,
    Headers,
    Body,
    Dependencies,
    Scripts,
    Docs,
    Config,
}

impl ButtonGroupInner for RequestInputTab {
    fn render(&self) -> Element {
        return rsx! {
            Label { "{self}" }
        };
    }
}

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display)]
enum RequestOutputTab {
    Request,
    Response,
}

impl ButtonGroupInner for RequestOutputTab {
    fn render(&self) -> Element {
        return rsx! {
            Label { "{self}" }
        };
    }
}

#[component]
pub fn RequestPage() -> Element {
    let _state = use_context::<TabState<WorkspaceTab>>();
    let mut url = use_signal(|| String::new());
    let method = use_signal(|| Some("GET".to_string()));
    let mut request_tab_value = use_signal(|| RequestInputTab::Params);
    let mut response_tab_value = use_signal(|| RequestOutputTab::Request);

    return rsx! {
        div { class: "h-full flex flex-col pt-2 gap-2",
            // method send and input
            TextField {
                value: "{url}",
                oninput: move |e: Event<FormData>| {
                    url.set(e.value());
                },
                placeholder: "https://httpbin.org/get",
                before: rsx! {
                    Select::<String> {
                        class: "w-full h-full",
                        dropdown_class: "!w-auto",
                        value: method,
                        placeholder: "--No method--",
                        items: HTTP_METHODS.iter().map(|i| i.to_string()).collect::<Vec<String>>(),
                    }
                },
                after: rsx! {
                    Button { style: ButtonStyleVariant::Secondary,
                        Label { "Send" }
                    }
                },
            }

            // Body
            Pane {
                class: "flex-grow flex gap-2 overflow-clip rounded-md",
                border: Border::all(),
                // Input
                div { class: "flex-grow flex-col gap-2 flex p-2",
                    ButtonGroup<RequestInputTab> {
                        class: "flex items-center gap-1",
                        value: request_tab_value(),
                        buttons: RequestInputTab::iter().collect::<Vec<RequestInputTab>>(),
                        onselect: move |v| {
                            request_tab_value.set(v);
                        }
                    }

                    Pane { class: "rounded-md flex-grow" }
                }

                // Output
                Pane {
                    class: "w-[35%] max-w-[500px] p-2",
                    // style: PaneStyleVariant::Darker,
                    border: Border::left(),

                    ButtonGroup<RequestOutputTab> {
                        class: "flex items-center gap-1",
                        value: response_tab_value(),
                        buttons: RequestOutputTab::iter().collect::<Vec<RequestOutputTab>>(),
                        onselect: move |v| {
                            response_tab_value.set(v);
                        }
                    }

                    Pane { class: "rounded-md flex-grow" }
                }
            }
        }
    };
}

use components_lib::{
    border::Border,
    button::{Button, ButtonStyleVariant},
    buttongroup::{ButtonGroup, GroupButton},
    label::Label,
    pane::{Pane},
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
enum RequestTab {
    Params,
    Authorization,
    Headers,
    Body,
    Dependencies,
    Scripts,
    Docs,
    Config,
}

#[component]
pub fn RequestPage() -> Element {
    let state = use_context::<TabState<WorkspaceTab>>();
    let url = use_signal(|| String::new());
    let method = use_signal(|| Some("GET".to_string()));

    return rsx! {
        div { class: "h-full flex flex-col pt-2 gap-2",
            // method send and input
            TextField {
                value: url,
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
            div { class: "flex-grow flex gap-2",
                div { class: "flex-grow flex-col gap-2 flex",
                    Pane { class: "p-1 rounded-md flex items-center",
                        ButtonGroup {
                            class: "flex items-center gap-1",
                            for tab in RequestTab::iter(){
                                GroupButton {
                                    key: "{tab}",
                                    Label { "{tab}" }
                                }
                            }
                        }
                    }

                    Pane { class: "rounded-md flex-grow", border: Border::all() }
                }
                Pane {
                    class: "w-[35%] max-w-[500px] rounded-md",
                    // style: PaneStyleVariant::Darker,
                    border: Border::all(),
                }
            }
        }
    };
}

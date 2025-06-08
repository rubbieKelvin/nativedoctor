use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};
use rustle_ui_components::keyvalue;

#[derive(PartialEq, Clone, Copy)]
enum RequestTab {
    Params,
    Authorization,
    Headers,
    Body,
}

impl RequestTab {
    fn all() -> Vec<RequestTab> {
        vec![
            RequestTab::Params,
            RequestTab::Authorization,
            RequestTab::Headers,
            RequestTab::Body,
        ]
    }

    fn to_string(&self) -> &'static str {
        match self {
            RequestTab::Params => "Params",
            RequestTab::Authorization => "Authorization",
            RequestTab::Headers => "Headers",
            RequestTab::Body => "Body",
        }
    }
}

#[component]
pub fn RequestSection() -> Element {
    let mut selected_tab = use_signal(|| RequestTab::Params);

    rsx! {
        div {
            class: "flex-grow",

            // Tabs
            div {
                class: "flex gap-2 items-center px-2 py-1",

                for tab in RequestTab::all() {
                    button {
                        class: "hover:bg-item-hover-bg text-sm rounded-md px-2 py-1",
                        class: if selected_tab() == tab { "bg-item-hover-bg text-accent" } else { "" },
                        onclick: move |_| {
                            selected_tab.set(tab);
                        },
                        "{tab.to_string()}"
                    }
                }

                div { class: "flex-grow", }

                button {
                    title: "Documentation",
                    class: "p-1 rounded hover:bg-item-hover-bg",
                    Icon {
                        icon: ld_icons::LdBook,
                        height: 14,
                        width: 14
                    }
                }

                button {
                    title: "Scripts",
                    class: "p-1 rounded hover:bg-item-hover-bg",
                    Icon {
                        icon: ld_icons::LdFileJson,
                        height: 14,
                        width: 14
                    }
                }

            }

            // Content
            match selected_tab() {
                RequestTab::Params => rsx! { ParamsSection {} },
                RequestTab::Headers => rsx! { HeadersSection {} },
                _ => rsx! { div { "Unimplemented Content goes here" } },
            }
        
        }
    }
}

#[component]
fn ParamsSection() -> Element {
    rsx! {
        div {
            class: "px-1",
            keyvalue::editor::KeyValueEditor {  }
        }
    }
}

#[component]
fn HeadersSection() -> Element {
    rsx! {
        div {
            class: "px-1",
            keyvalue::editor::KeyValueEditor {  }
        }
    }
}



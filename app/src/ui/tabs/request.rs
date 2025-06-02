use crate::appdata;
use crate::appdata::requests::{RequestItem, RequestManager};
use dioxus::prelude::*;
use dioxus_free_icons::icons::{hi_solid_icons, ld_icons};
use dioxus_free_icons::Icon;
use rustle_ui_components::select::Select;

#[component]
pub fn RequestPage(mut request: RequestItem) -> Element {
    let mut request_manager = RequestManager::inject();
    let mut selected_tab = use_signal(|| RequestTab::Params);
    let mut selected_bottom_tab = use_signal(|| BottomPaneTab::RequestData);
    let mut selected_method = use_signal(|| Some(appdata::requests::HttpMethod::GET));

    {
        let id = request.id.clone();
        use_effect(move || {
            let method = selected_method();
            if method.is_none() {
                return;
            }

            request_manager.with_mut(|m| {
                m.update(id.clone(), move |r| {
                    r.method = method.unwrap();
                });
            });
        });
    }

    rsx! {
        div {
            class: "flex flex-col h-full",
            div {
                class: "flex items-center gap-4 border-b",
                input {
                    class: "flex-grow h-full outline-none py-3 px-2",
                    placeholder: "Enter request name",
                    value: request.clone().name,
                    oninput: move |evt| {
                        let name = evt.value();
                        request_manager.with_mut(|m| {
                            m.update(request.id.clone(), move |r| {
                                r.name = name;
                            });
                        })
                    }
                }
                // button group
                div {
                    class: "flex gap-2 items-center px-4",

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
            }

            // URL and Method Section
            div {
                class: "flex items-center gap-4 border-b",
                Select<appdata::requests::HttpMethod> {
                    items: appdata::requests::HttpMethod::all(),
                    selected: selected_method,
                    render_selected: |method: &appdata::requests::HttpMethod| method.to_string(),
                    render_item: |method: &appdata::requests::HttpMethod| rsx! { div { class: "px-2 py-0.1", "{method.to_string()}" } },
                    placeholder: "Select method",
                    class: "text-sm ml-2",
                    wrapper_class: "w-24",
                    dropdown_class: "bg-bg-primary border rounded-md",
                    item_class: "hover:bg-item-hover-bg px-2 py-1",
                }
                input {
                    class: "flex-grow h-full outline-none py-3",
                    placeholder: "Enter request URL"
                }
                button {
                    class: "px-4 py-1 bg-accent hover:bg-accent/70 h-full",
                    "Send"
                }
            }

            // // Top Tabs Section
            // div {
            //     class: "flex border-b border-gray-200 dark:border-gray-700",
            //     TabButton { name: "Params", active_tab: selected_tab, tab: RequestTab::Params }
            //     TabButton { name: "Authorization", active_tab: selected_tab, tab: RequestTab::Authorization }
            //     TabButton { name: "Headers", active_tab: selected_tab, tab: RequestTab::Headers }
            //     TabButton { name: "Body", active_tab: selected_tab, tab: RequestTab::Body }
            //     TabButton { name: "Scripts", active_tab: selected_tab, tab: RequestTab::Scripts }
            //     TabButton { name: "Documentation", active_tab: selected_tab, tab: RequestTab::Documentation }
            // }

            // // Top Tab Content Section
            // div {
            //     class: "flex-grow p-4",
            //     match selected_tab() {
            //         RequestTab::Params => rsx! { KeyValueEditor { title: "Query Params" } },
            //         RequestTab::Authorization => rsx! { div { "Authorization content goes here" } },
            //         RequestTab::Headers => rsx! { KeyValueEditor { title: "Headers" } },
            //         RequestTab::Body => rsx! { div { "Body content goes here" } },
            //         RequestTab::Scripts => rsx! { div { "Scripts content goes here" } },
            //         RequestTab::Documentation => rsx! { div { "Documentation content goes here" } },
            //     }
            // }

            // // Bottom Tab Group
            // div {
            //     class: "h-1/3 border-t border-gray-200 dark:border-gray-700 flex flex-col",
            //     // Bottom Tab Buttons
            //     div {
            //         class: "flex border-b border-gray-200 dark:border-gray-700",
            //         BottomTabButton { name: "Request Data", active_tab: selected_bottom_tab, tab: BottomPaneTab::RequestData }
            //         BottomTabButton { name: "Console Log", active_tab: selected_bottom_tab, tab: BottomPaneTab::ConsoleLog }
            //     }
            //     // Bottom Tab Content
            //     div {
            //         class: "flex-grow p-4 bg-gray-50 dark:bg-gray-800 overflow-auto",
            //         match selected_bottom_tab() {
            //             BottomPaneTab::RequestData => rsx! { div { "Request Data (Body, Cookies, Headers) content goes here" } },
            //             BottomPaneTab::ConsoleLog => rsx! { div { "Console Log content goes here" } },
            //         }
            //     }
            // }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum RequestTab {
    Params,
    Authorization,
    Headers,
    Body,
    Scripts,
    Documentation,
}

#[derive(PartialEq, Clone, Copy)]
enum BottomPaneTab {
    RequestData,
    ConsoleLog,
}

#[derive(Props, PartialEq, Clone)]
struct KeyValueItem {
    item_key: String,
    value: String,
}

#[component]
fn KeyValueEditor(title: &'static str) -> Element {
    let mut items = use_signal(|| {
        vec![KeyValueItem {
            item_key: "".to_string(),
            value: "".to_string(),
        }]
    });

    rsx! {
        div {
            class: "flex flex-col h-full",
            h2 {
                class: "text-lg font-semibold mb-2 dark:text-white",
                "{title}"
            }
            div {
                class: "flex-grow overflow-auto",
                {items.read().iter().enumerate().map(|(index, _item)| rsx! {
                    div {
                        class: "flex items-center mb-2",
                        key: format_args!("kv-item-{}", index),
                        input {
                            class: "flex-grow p-2 border border-gray-300 rounded-l dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            placeholder: "Key",
                            value: "{items.read()[index].item_key}",
                            oninput: move |evt| {
                                let mut current_items = items.read().clone();
                                current_items[index].item_key = evt.value();
                                items.set(current_items);
                            }
                        }
                        input {
                            class: "flex-grow p-2 border-t border-b border-r border-gray-300 rounded-r dark:bg-gray-700 dark:border-gray-600 dark:text-white",
                            placeholder: "Value",
                            value: "{items.read()[index].value}",
                            oninput: move |evt| {
                                let mut current_items = items.read().clone();
                                current_items[index].value = evt.value();
                                items.set(current_items);
                            }
                        }
                        button {
                            class: "ml-2 p-2 text-red-500 hover:text-red-700 dark:text-red-400 dark:hover:text-red-300",
                            onclick: move |_| {
                                let mut current_items = items.read().clone();
                                current_items.remove(index);
                                if current_items.is_empty() {
                                    current_items.push(KeyValueItem { item_key: "".to_string(), value: "".to_string() });
                                }
                                items.set(current_items);
                            },
                            "Remove"
                        }
                    }
                })}
            }
            button {
                class: "mt-2 p-2 bg-green-500 text-white rounded hover:bg-green-600 self-start",
                onclick: move |_| {
                    let mut current_items = items.read().clone();
                    current_items.push(KeyValueItem { item_key: "".to_string(), value: "".to_string() });
                    items.set(current_items);
                },
                if title == "Query Params" { "Add Param" } else { "Add Header" }
            }
        }
    }
}

#[component]
fn TabButton<T: PartialEq + Clone + Copy + 'static>(
    name: &'static str,
    active_tab: Signal<T>,
    tab: T,
) -> Element {
    let base_class = "px-4 py-2 cursor-pointer focus:outline-none";
    let active_class =
        "border-b-2 border-blue-500 text-blue-500 dark:text-blue-400 dark:border-blue-400";
    let inactive_class =
        "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200";

    let current_class = if active_tab() == tab {
        format!("{} {}", base_class, active_class)
    } else {
        format!("{} {}", base_class, inactive_class)
    };

    rsx! {
        button {
            class: "{current_class}",
            onclick: move |_| active_tab.set(tab),
            "{name}"
        }
    }
}

#[component]
fn BottomTabButton<T: PartialEq + Clone + Copy + 'static>(
    name: &'static str,
    active_tab: Signal<T>,
    tab: T,
) -> Element {
    rsx! { TabButton { name: name, active_tab: active_tab, tab: tab } }
}

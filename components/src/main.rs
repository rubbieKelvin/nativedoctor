use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};
use strum::IntoEnumIterator;

use crate::{tabs::TabPayload, toast::use_toast};

mod border;
mod button;
mod buttongroup;
mod contextmenu;
mod label;
mod numberfield;
mod pane;
mod tableinput;
mod tabs;
mod textfield;
mod toast;
mod traits;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_JS: Asset = asset!("/assets/tailwind.js");

#[derive(PartialEq, Clone)]
struct TabBook {
    name: String,
    description: String,
}

impl TabPayload for TabBook {
    type Identifier = String;

    fn render_title(&self, _selected: bool) -> Element {
        return rsx! {
            label::Label {
                class: "flex-grow text-start",
                style: label::LabelStyleVariant::Mild,
                "{self.name}"
            }
        };
    }

    fn unique_identifier(&self) -> Self::Identifier {
        return self.name.clone();
    }
}

#[component]
fn Tabs() -> Element {
    let tablist: Signal<tabs::TabSet<TabBook>> = use_signal(|| {
        tabs::TabSet::new(vec![
            tabs::TabItemData::new(TabBook {
                name: "Rubbie".to_string(),
                description: "Rubbie is rubbie the one".to_string(),
            }),
            tabs::TabItemData::new(TabBook {
                name: "Bank".to_string(),
                description: "The bank is where you store money".to_string(),
            }),
            tabs::TabItemData::new(TabBook {
                name: "Rust".to_string(),
                description: "Rust is a fucked up language. but i like it".to_string(),
            }),
        ])
    });

    return rsx! {
        for orientation in tabs::TabOrientationVariant::iter() {

            tabs::TabsManager {
                class: "border border-[#3b3b3b] p-1 rounded-md gap-2",
                tabs: tablist,
                orientation,
                _TabContent {}
            }
        }
    };
}

#[component]
fn _TabContent() -> Element {
    let state = use_context::<tabs::TabState<TabBook>>();
    let name = state.tab.payload.name;
    let description = state.tab.payload.description;

    return rsx! {
        div { class: "border border-[#3b3b3b] w-full h-full rounded-md p-2",
            label::Label { size: label::LabelSizeVariant::Large, "{name}" }
            label::Label { size: label::LabelSizeVariant::Small, "{description}" }
        }
    };
}

#[derive(PartialEq, Clone, strum::Display)]
enum Columns {
    Identifier,
    Value,
}

impl tableinput::TableInputCell for Columns {
    fn identifier(&self) -> String {
        return self.to_string();
    }

    fn render_header(&self) -> Element {
        return rsx! {
            p { class: "text-start", "{self}" }
        };
    }

    fn render_input(
        &self,
        value: tableinput::CellValue,
        set: impl Fn(tableinput::CellValue) + 'static,
    ) -> Element {
        return match self {
            Columns::Identifier => rsx! {
                textfield::TextField {
                    size: textfield::TextFieldSizeVariant::Small,
                    value: value.to_string(),
                    oninput: move |e: Event<FormData>| {
                        let value = e.value();
                        let value = value.trim();
                        if value.is_empty() {
                            set(tableinput::CellValue::Empty)
                        } else {
                            set(tableinput::CellValue::Text(value.to_string()))
                        }
                    },
                }
            },
            Columns::Value => rsx! {
                numberfield::NumberField {
                    value: value.to_i64().map(|i| i as i32),
                    onchange: move |e: i32| {
                        let value: i64 = e.into();
                        if value == 0 {
                            set(tableinput::CellValue::Empty)
                        } else {
                            set(tableinput::CellValue::Number(value));
                        }
                    },
                }
            },
        };
    }
}

#[component]
fn TableInputs() -> Element {
    let mut rows = use_signal::<Vec<HashMap<String, tableinput::CellValue>>>(|| {
        vec![HashMap::from_iter(vec![
            (
                "Identifier".to_string(),
                tableinput::CellValue::Text("Hello rubbie".to_string()),
            ),
            ("Value".to_string(), tableinput::CellValue::Number(5)),
        ])]
    });
    let text = format!("{:?}", rows());

    return rsx! {
        h1 { "Table input" }

        label::Label {
            size: label::LabelSizeVariant::Tiny,
            style: label::LabelStyleVariant::Mild,
            "{text}"
        }

        tableinput::TableInput {
            class: "rounded-md p-2",
            border: Some(border::Border::all()),
            value: rows(),
            columns: vec![Columns::Identifier, Columns::Value],
            onchange: move |new_value| {
                let mut rows = rows.write();
                *rows = new_value;
            },
        }

        tableinput::TableInput {
            class: "rounded-md p-2",
            border: Some(border::Border::all()),
            value: rows(),
            columns: vec![Columns::Identifier, Columns::Value],
            onchange: move |new_value| {
                let mut rows = rows.write();
                *rows = new_value;
            },
        }
    };
}

#[component]
fn Toasts() -> Element {
    let mut toast = use_toast();

    return rsx! {
        div {
            h1 { "Toasts" }

            div { class: "flex gap-2",
                for variant in toast::ToastVariant::iter() {
                    button::Button {
                        onclick: move |_| {
                            toast
                                .push(
                                    toast::ToastConfig::new(
                                        variant.to_string(),
                                        variant.clone(),
                                        toast::ToastCloseMethod::Button,
                                    ),
                                );
                        },
                        "{variant}"
                    }
                }
            }
        }
    };
}

#[component]
fn Buttons() -> Element {
    return rsx! {
        div { class: "flex flex-col gap-2",
            h1 { "Buttons" }

            for size in button::ButtonSizeVariant::iter() {
                div { key: "{size}", class: "flex gap-2",
                    p { class: "text-sm text-gray-400", "{size}" }
                    for style in button::ButtonStyleVariant::iter() {
                        button::Button {
                            key: "{style}",
                            style: style.clone(),
                            size: size.clone(),
                            class: "flex items-center justify-center",

                            if size != button::ButtonSizeVariant::Icon {
                                "{style}"
                            } else {
                                Icon {
                                    width: 16,
                                    height: 16,
                                    icon: ld_icons::LdHome,
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn Labels() -> Element {
    return rsx! {
        div { class: "flex gap-2 flex-col",
            h1 { "Label" }

            div { class: "flex gap-4",

                for style in label::LabelStyleVariant::iter() {
                    div {
                        for size in label::LabelSizeVariant::iter() {
                            label::Label { size: size.clone(), style: style.clone(), "{size.to_string()}" }
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn Panes() -> Element {
    let menu: Vec<contextmenu::MenuItem> = vec![
        contextmenu::MenuItem {
            label: "Copy".to_string(),
            onclick: Some(EventHandler::new(move |_| tracing::info!("Copy clicked!"))),
            icon: Some(rsx! {
                Icon{
                    height: 14,
                    width: 14,
                    icon: ld_icons::LdCopy,
                }
            }),
            ..contextmenu::MenuItem::default() // Use default for other fields
        },
        contextmenu::MenuItem {
            label: "Paste".to_string(),
            onclick: Some(EventHandler::new(move |_| tracing::info!("Paste clicked!"))),
            icon: Some(rsx! {
                Icon{
                    height: 14,
                    width: 14,
                    icon: ld_icons::LdClipboardPaste,
                }
            }),
            ..contextmenu::MenuItem::default()
        },
        contextmenu::MenuItem {
            label: "Share".to_string(),
            disabled: true, // This item will be unclickable
            icon: Some(rsx! {
                Icon{
                    height: 14,
                    width: 14,
                    icon: ld_icons::LdShare2,
                }
            }),
            ..contextmenu::MenuItem::default()
        },
        // An item without an icon
        contextmenu::MenuItem {
            label: "Delete".to_string(),
            onclick: Some(EventHandler::new(move |_| {
                tracing::info!("Delete clicked!")
            })),
            ..contextmenu::MenuItem::default()
        },
    ];

    return rsx! {
        div { class: "flex gap-2 flex-col",
            h1 { "Pane" }

            div { class: "flex gap-2",
                for style in pane::PaneStyleVariant::iter() {
                    contextmenu::ContextMenu { items: menu.clone(),
                        pane::Pane { class: "p-8 rounded-md", style: style.clone(), "{style}" }
                    }
                }
            }
        }
    };
}

#[component]
fn ButtonGroups() -> Element {
    return rsx! {
        div { class: "flex gap-2 flex-col",
            h1 { "Button group" }

            div {
                h1 { "Single select" }

                buttongroup::ButtonGroup {
                    buttongroup::GroupButton {
                        label::Label { "Group 1" }
                    }

                    buttongroup::GroupButton {
                        label::Label { "Group 2" }
                    }

                    buttongroup::GroupButton {
                        label::Label { "Group 3" }
                    }
                }
            }

            div {
                h1 { "Multi select" }

                buttongroup::ButtonGroup {
                    class: "flex gap-2",
                    multiselect: true,
                    active_style: button::ButtonStyleVariant::Secondary,

                    buttongroup::GroupButton {
                        label::Label { "Group 1" }
                    }

                    buttongroup::GroupButton {
                        label::Label { "Group 2" }
                    }

                    buttongroup::GroupButton {
                        label::Label { "Group 3" }
                    }
                }
            }
        }
    };
}

#[component]
fn TextFields() -> Element {
    let mut text = use_signal(|| String::new());

    return rsx! {
        div {
            h1 {
                "Text field ("
                {text}
                ")"
            }

            textfield::TextField {
                value: "{text}",
                oninput: move |e: Event<FormData>| {
                    text.set(e.value());
                },
                before: rsx! {
                    p { "X" }
                },
            }

            for size in textfield::TextFieldSizeVariant::iter() {
                div { class: "flex",
                    for style in textfield::TextFieldStyleVariant::iter() {
                        textfield::TextField {
                            value: "{text}",
                            style,
                            size: size.clone(),
                            oninput: move |e: Event<FormData>| {
                                text.set(e.value());
                            },
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn NumberInputs() -> Element {
    let mut value = use_signal(|| 0);

    return rsx! {
        div {
            h1 { "Number input" }
            div { class: "flex flex-col gap-2",
                for size in numberfield::NumberFieldSizeVariant::iter() {
                    div { class: "flex gap-2",
                        for style in numberfield::NumberFieldStyleVariant::iter() {
                            numberfield::NumberField {
                                class: "flex-grow",
                                style,
                                value: value(),
                                size: size.clone(),
                                onchange: move |e: i32| {
                                    value.set(e);
                                },
                            }
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn App() -> Element {
    return rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: TAILWIND_JS }
        div { class: "flex gap-4 flex-col p-4",
            h1 { class: "mb-4", "Preview" }
            TableInputs {}
            Tabs {}
            toast::ToastProvider { Toasts {} }
            TextFields {}
            NumberInputs {}
            Buttons {}
            Labels {}
            Panes {}
            ButtonGroups {}
        }
    };
}

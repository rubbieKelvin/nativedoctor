use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};
use strum::IntoEnumIterator;

mod button;
mod buttongroup;
mod label;
mod numberfield;
mod pane;
mod textfield;
mod traits;

fn main() {
    dioxus::launch(App);
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_JS: Asset = asset!("/assets/tailwind.js");

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
    return rsx! {
        div { class: "flex gap-2 flex-col",
            h1 { "Pane" }

            div { class: "flex gap-2",
                for style in pane::PaneStyleVariant::iter() {
                    pane::Pane { class: "p-8 rounded-md", style: style.clone(), "{style}" }
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
    let text = use_signal(|| String::new());


    return rsx! {
        div {
            h1 { "Text field ("{text}")" }
            for size in textfield::TextFieldSizeVariant::iter() {
                div {
                    for style in textfield::TextFieldStyleVariant::iter() {
                        textfield::TextField {
                            value: text,
                            style,
                            size: size.clone()
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn NumberInputs() -> Element {
    let value = use_signal(|| 0);

    return rsx!{
        div {
            h1{"Number input"}
            div {
                class: "flex flex-col gap-2",
                for size in numberfield::NumberFieldSizeVariant::iter(){
                    div {
                        class: "flex gap-2",
                        for style in numberfield::NumberFieldStyleVariant::iter(){
                            numberfield::NumberField {
                                class: "flex-grow",
                                style,
                                value,
                                size: size.clone()
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

            NumberInputs {  }
            Buttons {}
            Labels {}
            Panes {}
            ButtonGroups {}
            TextFields {}
        }
    };
}

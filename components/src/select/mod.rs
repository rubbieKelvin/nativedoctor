use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};

use crate::{
    border::Border,
    button::Button,
    label::Label,
    pane::{Pane, PaneStyleVariant},
};

#[derive(PartialEq, Clone)]
pub struct SelectState<T: Clone + PartialEq + Into<String> + 'static> {
    pub item: T,
    pub selected: bool,
}

impl<T: Clone + PartialEq + Into<String> + 'static> SelectState<T> {
    pub fn new(item: T, selected: bool) -> SelectState<T> {
        return SelectState { item, selected };
    }
}

#[component]
pub fn Select<T: Clone + PartialEq + Into<String> + 'static>(
    items: Vec<T>,
    class: Option<String>,
    value: Signal<Option<T>>,
    placeholder: Option<String>,
    wrapper_class: Option<String>,
    dropdown_class: Option<String>,
    item_class: Option<String>,
    display: Option<Element>,
    child: Option<Element>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let placeholder = placeholder.unwrap_or_else(|| "Select".to_string());
    let class = class.unwrap_or_default();
    let item_class = item_class.unwrap_or_default();
    let dropdown_class = dropdown_class.unwrap_or_default();
    let wrapper_class = wrapper_class.unwrap_or_default();

    rsx! {
        div { class: format!("relative text-left {}", wrapper_class),

            // The dropdown toggle button
            if let Some(display) = display {
                div { tabindex: 0, onclick: move |_| is_open.set(!is_open()), {display} }
            } else {
                // We'd should a default if display is not set
                Button {
                    class: "flex {class} item-center gap-2",
                    onclick: move |_| is_open.set(!is_open()),
                    Label { class: "flex-grow text-start",
                        if let Some(v) = &value() {
                            "{v.clone().into()}"
                        } else {
                            "{placeholder}"
                        }
                    }

                    // Dropdown chevron icon
                    Icon {
                        icon: ld_icons::LdChevronDown,
                        width: 14,
                        height: 14,
                        class: if is_open() { "transform rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" },
                    }
                }
            }


            // Dropdown menu
            if is_open() {
                // Full-screen backdrop
                div {
                    class: "fixed inset-0 z-10",
                    onclick: move |_| is_open.set(false),
                }

                // The dropdown menu itself
                Pane {
                    class: format!(
                        "origin-top-right w-full absolute left-0 mt-2 rounded-md shadow-lg focus:outline-none z-20 {}",
                        dropdown_class,
                    ),
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_labelledby: "menu-button",
                    tabindex: -1,
                    style: PaneStyleVariant::Dark,
                    border: Border::all(),

                    div { class: "py-0.5",

                        for item in items.iter() {
                            div {
                                class: item_class.clone(),
                                role: "menuitem",
                                tabindex: "-1",
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        value.set(Some(item_clone.clone()));
                                        is_open.set(false);
                                    }
                                },
                                SelectItem {
                                    state: SelectState::new(item.clone(), value().is_some() && value() == Some(item.clone())),
                                    child: child.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// selection item. we'd pass the current item as a context for other component outside of this to pick up
#[component]
fn SelectItem<T: Clone + PartialEq + Into<String> + 'static>(
    state: SelectState<T>,
    child: Option<Element>,
) -> Element {
    use_context_provider::<SelectState<T>>(|| state.clone());

    let item = state.item.clone();
    let item_string: String = item.clone().into();
    let class = if state.selected {
        "bg-[#245c80]"
    } else {
        "hover:bg-[#3d3d3d] bg-transparent"
    };

    return match child {
        Some(child) => {
            rsx! {
                {child}
            }
        }
        None => {
            // render default item
            rsx! {
                div { class: "w-full {class}",
                    Label { class: "px-2 py-0.5", "{item_string}" }
                }
            }
        }
    };
}

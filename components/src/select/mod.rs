use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(
    items: Vec<T>,
    selected: Signal<Option<T>>,
    render_selected: fn(&T) -> String,
    render_item: fn(&T) -> Element,
    placeholder: Option<&'static str>,
    class: Option<&'static str>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut selected_signal = selected;
    let placeholder = placeholder.unwrap_or("Select an option");
    let class = class.unwrap_or("");

    rsx! {
        div {
            class: "relative inline-block text-left z-20",

            // The dropdown toggle button
            button {
                class: format!("
                    border rounded-md py-1 px-2 flex
                    items-center justify-between gap-2
                    transition-colors focus:outline-none
                    focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2
                    w-full {}", class),
                onclick: move |_| is_open.set(!is_open()),

                // Display selected value or placeholder
                if let Some(selected) = selected_signal() {
                    p {
                        class: "whitespace-nowrap",
                        "{(render_selected)(&selected)}"
                    }
                } else {
                    p { class: "whitespace-nowrap text-gray-500", "{placeholder}" }
                }

                // Dropdown chevron icon
                Icon {
                    icon: ld_icons::LdChevronDown,
                    width: 16,
                    height: 16,
                    class: if is_open() {
                        "transform rotate-180 transition-transform duration-200"
                    } else {
                        "transition-transform duration-200"
                    },
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
                div {
                    class: "origin-top-right w-full absolute left-0 mt-2 rounded-md shadow-lg bg-bg-secondary ring-1 ring-black ring-opacity-5 focus:outline-none z-20",
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_labelledby: "menu-button",
                    tabindex: "-1",

                    div {
                        class: "py-1",
                        role: "none",

                        for item in items {
                            div {
                                class: "hover:bg-item-hover-bg cursor-pointer",
                                role: "menuitem",
                                tabindex: "-1",
                                onclick: {
                                    let item_clone = item.clone();
                                    move |_| {
                                        selected_signal.set(Some(item_clone.clone()));
                                        is_open.set(false);
                                    }
                                },
                                {(render_item)(&item)}
                            }
                        }
                    }
                }
            }
        }
    }
}

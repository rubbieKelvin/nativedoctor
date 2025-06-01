use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[derive(Props, Clone, PartialEq)]
pub struct SelectProps<T: Clone + PartialEq + 'static> {
    /// The list of items to display in the dropdown
    pub items: Vec<T>,

    /// The currently selected item (as a signal)
    pub selected: Signal<Option<T>>,

    /// Function to render the selected item in the button
    pub render_selected: fn(&T) -> String,

    /// Function to render each item in the dropdown
    pub render_item: fn(&T) -> Element,

    /// Optional placeholder text when nothing is selected
    #[props(default = "Select an option")]
    pub placeholder: &'static str,

    /// Optional custom class for the select button
    #[props(default = "")]
    pub class: &'static str,
}

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let mut is_open = use_signal(|| false);
    let mut selected_signal = props.selected;

    rsx! {
        // Main container for the dropdown
        div {
            class: "relative inline-block text-left z-20 w-full",

            // The dropdown toggle button
            button {
                class: format!("border rounded-md py-1 px-2 flex items-center justify-between gap-2 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 w-full min-w-[120px] {}", props.class),
                onclick: move |_| is_open.set(!is_open()),

                // Display selected value or placeholder
                p {
                    class: "whitespace-nowrap text-sm font-medium",
                    if let Some(selected) = selected_signal() {
                        "{(props.render_selected)(&selected)}"
                    } else {
                        "{props.placeholder}"
                    }
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
                    class: "origin-top-right absolute left-0 mt-2 w-full rounded-md shadow-lg bg-bg-secondary ring-1 ring-black ring-opacity-5 focus:outline-none z-20",
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_labelledby: "menu-button",
                    tabindex: "-1",

                    div {
                        class: "py-1",
                        role: "none",

                        for item in props.items {
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
                                {(props.render_item)(&item)}
                            }
                        }
                    }
                }
            }
        }
    }
}

// Example usage with a simple string select
#[component]
pub fn StringSelect(
    items: Vec<String>,
    selected: Signal<Option<String>>,
    placeholder: Option<&'static str>,
) -> Element {
    rsx! {
        Select {
            items,
            selected,
            render_selected: |s| s.clone(),
            render_item: |s| rsx! {
                div {
                    class: "px-4 py-2",
                    "{s}"
                }
            },
            placeholder: placeholder.unwrap_or("Select an option"),
        }
    }
}

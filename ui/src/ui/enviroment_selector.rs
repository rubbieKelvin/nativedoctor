use crate::appdata::Environment;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn EnviromentSelector() -> Element {
    let environments = use_context::<Signal<Vec<Environment>>>();
    let mut is_open = use_signal(|| false);

    // State to store the currently selected environment
    let mut selected_environment = use_signal(|| environments()[0].name.clone()); // Default to the first environment

    rsx! {
        // Main container for the dropdown. Relative positioning needed for absolute menu.
        div {
            class: "relative inline-block text-left z-20 w-full",

            // The dropdown toggle button
            button {
                class: "border rounded-md py-1 px-2 flex items-center justify-between gap-2 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 w-full min-w-[120px]",
                onclick: move |_| is_open.set(!is_open()),
                // Display
                p {
                    class: "whitespace-nowrap text-sm font-medium ",
                    "{selected_environment}"
                }
                Icon {
                    icon: ld_icons::LdChevronDown,
                    width: 16,
                    height: 16,
                    class: if is_open() { "transform rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" },
                }
            }

            //
            if is_open() {
                // full-screen backdrop
                div {
                    class: "fixed inset-0 z-10", // Fixed position, covers entire viewport, lower z-index than menu
                    onclick: move |_| is_open.set(false),
                }

                // The dropdown menu itself
                div {
                    class: "origin-top-right absolute left-0 mt-2 w-full rounded-md shadow-lg bg-bg-secondary ring-1 ring-black ring-opacity-5 focus:outline-none z-20", // Higher z-index than backdrop
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_labelledby: "menu-button",
                    tabindex: "-1",

                    div {
                        class: "py-1",
                        role: "none",
                        for env in environments() {
                            div {
                                class: "hover:bg-item-hover-bg flex flex-col gap-0 px-4 py-2",
                                p{
                                class: "  ",
                                role: "menuitem",
                                tabindex: "-1",
                                onclick: move |_| {
                                    selected_environment.set(env.name.clone()); // Update selected environment
                                    is_open.set(false);
                                },
                                "{env.name}"
                            }
                            p {
                                class: "text-gray-400 italic text-sm",
                                "{env.description}"
                            }
                        }
                        }
                    }
                }
            }
        }
    }
}

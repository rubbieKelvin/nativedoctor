use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

// Define the available environments
const ENVIRONMENTS: &[&str] = &["Development", "Staging", "Production", "Testing"];

#[component]
pub fn EnviromentSelector() -> Element {
    // State to manage if the dropdown is open or closed
    let mut is_open = use_signal(|| false);

    // State to store the currently selected environment
    let mut selected_environment = use_signal(|| ENVIRONMENTS[0].to_string()); // Default to the first environment

    rsx! {
        // Main container for the dropdown. Relative positioning needed for absolute menu.
        div {
            class: "relative inline-block text-left z-20",

            // The dropdown toggle button
            button {
                class: "border rounded-md py-1 px-2 flex items-center justify-between gap-2 bg-white hover:bg-gray-100 transition-colors focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 w-full min-w-[120px]",
                onclick: move |_| is_open.set(!is_open()), // Toggle dropdown open/closed
                p {
                    class: "whitespace-nowrap text-sm font-medium text-gray-700",
                    "{selected_environment}" // Display the currently selected environment
                }
                Icon {
                    icon: ld_icons::LdChevronDown,
                    width: 16,
                    height: 16,
                    // Rotate the icon based on whether the dropdown is open
                    class: if *is_open.read() { "transform rotate-180 transition-transform duration-200" } else { "transition-transform duration-200" },
                }
            }

            // --- The Backdrop and Dropdown Menu ---
            if *is_open.read() {
                // The full-screen backdrop that closes the dropdown when clicked
                div {
                    class: "fixed inset-0 z-10", // Fixed position, covers entire viewport, lower z-index than menu
                    onclick: move |_| is_open.set(false), // Clicking anywhere on the backdrop closes the dropdown
                }

                // The dropdown menu itself
                div {
                    class: "origin-top-right absolute left-0 mt-2 w-full rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-20", // Higher z-index than backdrop
                    role: "menu",
                    aria_orientation: "vertical",
                    aria_labelledby: "menu-button",
                    tabindex: "-1", // Make the menu focusable

                    div {
                        class: "py-1",
                        role: "none",
                        for env in ENVIRONMENTS.iter() {
                            a {
                                class: "text-gray-700 block px-4 py-2 text-sm hover:bg-indigo-500 hover:text-white cursor-pointer transition-colors",
                                role: "menuitem",
                                tabindex: "-1",
                                onclick: move |_| {
                                    selected_environment.set(env.to_string()); // Update selected environment
                                    is_open.set(false); // Close the dropdown after selection
                                },
                                "{env}"
                            }
                        }
                    }
                }
            }
        }
    }
}

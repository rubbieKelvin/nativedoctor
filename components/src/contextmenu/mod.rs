// main.rs
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};

// To see the output of the on_change handlers, run with:
// cargo watch -x "run -- -Z unstable-options --log-level=info"
// Or just check your browser's console.

/// Represents a single item in the context menu.
/// It's clonable and uses EventHandler for safe, repeatable callbacks.
#[derive(Props, PartialEq, Clone, Default)]
pub struct MenuItem {
    /// The text to display for the menu item.
    pub label: String,
    /// The callback to execute when the item is clicked.
    #[props(optional)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    /// Whether the item is disabled and cannot be clicked.
    #[props(default = false)]
    pub disabled: bool,
    /// An optional icon to display next to the label.
    /// This should be an `Icon` from a library like `dioxus-free-icons`.
    #[props(optional)]
    pub icon: Option<Element>,
}

#[component]
pub fn ContextMenu(
    class: Option<&'static str>,
    items: Vec<MenuItem>,
    children: Element,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut position = use_signal(|| (0, 0));
    let class = class.unwrap_or_default();

    rsx! {
        // The main container that wraps your content
        div {
            class: "relative",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    is_open.set(false);
                }
            },
            oncontextmenu: move |e| {
                e.prevent_default();
                if is_open() {
                    is_open.set(false);
                    return;
                }
                let rect = e.client_coordinates();
                position.set((rect.x as i32, rect.y as i32));
                is_open.set(true);
            },

            // Render the content that triggers the menu
            {children}

            // The actual context menu, rendered conditionally
            if is_open() {
                // A full-screen, invisible backdrop. Clicking it closes the menu.
                div {
                    class: "fixed inset-0 z-10",
                    onclick: move |_| is_open.set(false),
                    oncontextmenu: move |e| {
                        e.prevent_default();
                        is_open.set(false);
                    },
                }

                // The menu container itself, positioned at the mouse click.
                div {
                    // These class names look like they're from a utility CSS framework like Tailwind.
                    // You would need to have these styles defined in your project.
                    class: format!(
                        "fixed z-20 bg-[#1b1b1b] border border-[#3e3e3e] rounded shadow min-w-[180px] {}",
                        class,
                    ),
                    style: "left: {position().0}px; top: {position().1}px;",
                    // Stop click events from bubbling up to the backdrop
                    onclick: move |e| e.stop_propagation(),

                    div { class: "py-1",

                        // Loop through the provided items and render each one
                        for item in items {
                            div {
                                class: format!(
                                    "px-2 py-1 flex items-center gap-3 cursor-pointer hover:bg-gray-700 {}",
                                    if item.disabled { "opacity-50 cursor-not-allowed" } else { "" },
                                ),
                                // The click handler now correctly uses the clonable EventHandler
                                onclick: move |evt| {
                                    evt.stop_propagation();
                                    if !item.disabled {
                                        if let Some(handler) = &item.onclick {
                                            handler.call(evt);
                                        }
                                        is_open.set(false);
                                    }
                                },

                                // Render the icon if it exists
                                if let Some(icon) = item.icon {
                                    {icon}
                                } else {
                                    div { class: "w-4 h-4" }
                                }

                                // Render the label
                                span { class: "text-xs text-gray-200", {item.label} }
                            }
                        }
                    }
                }
            }
        }
    }
}

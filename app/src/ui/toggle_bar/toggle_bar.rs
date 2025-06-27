use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn ToggleBar(
    title: String,
    class: Option<String>,
    add_button: Option<Element>,
    body: Option<Element>,
    open: Option<Signal<bool>>,
    icon: Option<Element>,
    can_toggle: Option<bool>,
) -> Element {
    let mut open = open.unwrap_or_else(|| use_signal(|| true));
    let class = class.unwrap_or("".to_string());

    let icon = icon.unwrap_or_else(|| {
        if open() {
            rsx! {
                Icon {
                    icon: ld_icons::LdChevronDown,
                    width: 14,
                    height: 14,
                }
            }
        } else {
            rsx! {
                Icon {
                    icon: ld_icons::LdChevronRight,
                    width: 14,
                    height: 14,
                }
            }
        }
    });

    return rsx! {
        div {
            class: "border-b {class}",
            div {
                class: "flex items-center gap-1 px-2 py-1 sticky top-0 bg-inherit",
                button {
                    class: "flex items-center gap-2 flex-grow",
                    onclick: move |_| {
                        if can_toggle.unwrap_or(true) {
                            open.set(!open())
                        }
                    },

                    {icon}

                    p {
                        class: "",
                        "{title}"
                    }
                }

                if let Some(add_button) = add_button {
                    {add_button}
                }
            }

            if open() {
                if let Some(body) = body {
                    {body}
                }
            }
        }
    };
}

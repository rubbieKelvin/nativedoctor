use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon, IconShape};

#[component]
pub fn ToggleBar(title: String, add_button: Option<Element>, body: Option<Element>) -> Element {
    let mut open = use_signal(|| true);

    return rsx! {
        div {
            class: "border-b",
            div {
                class: "flex items-center gap-1 px-2 py-1 sticky top-0",
                button {
                    class: "flex items-center gap-2 flex-grow",
                    onclick: move |_| open.set(!open()),

                    if open() {
                        Icon {
                            icon: ld_icons::LdChevronDown,
                            width: 16,
                            height: 16,
                        }
                    } else {
                        Icon {
                            icon: ld_icons::LdChevronRight,
                            width: 16,
                            height: 16,
                        }
                    }
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

use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};

#[derive(Props, PartialEq, Clone)]
pub struct ContextMenuProps {
    #[props(default)]
    pub class: Option<&'static str>,
    pub items: Vec<MenuItem>,
    pub children: Element,
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuItem {
    pub label: String,
    pub on_click: EventHandler<()>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub icon: Option<Element>,
}

#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut position = use_signal(|| (0, 0));
    let class = props.class.unwrap_or("");

    rsx! {
        div {
            class: format!("relative {}", class),
            oncontextmenu: move |e| {
                e.prevent_default();
                let rect = e.client_coordinates();
                position.set((rect.x as i32, rect.y as i32));
                is_open.set(true);
            },

            // Render children
            {props.children}

            // Context menu
            if is_open() {
                // Backdrop to close menu when clicking outside
                div {
                    class: "fixed inset-0 z-10",
                    onclick: move |_| is_open.set(false),
                }

                // Menu container
                div {
                    class: "fixed z-20 bg-bg-secondary rounded-md shadow-lg ring-1 ring-black ring-opacity-5 min-w-[160px]",
                    style: format!("left: {}px; top: {}px;", position().0, position().1),
                    onclick: move |e| e.stop_propagation(),

                    div {
                        class: "py-1",

                        for item in props.items {
                            div {
                                class: format!(
                                    "px-4 py-2 flex items-center gap-2 cursor-pointer hover:bg-item-hover-bg {}",
                                    if item.disabled { "opacity-50 cursor-not-allowed" } else { "" }
                                ),
                                onclick: move |e| {
                                    e.stop_propagation();
                                    if !item.disabled {
                                        item.on_click.call(());
                                        is_open.set(false);
                                    }
                                },

                                if let Some(icon) = &item.icon {
                                    {icon.clone()}
                                }

                                span {
                                    class: "text-sm",
                                    {item.label}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

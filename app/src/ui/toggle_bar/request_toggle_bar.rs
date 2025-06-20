use super::toggle_bar;
use crate::appdata::requests::{RequestItem, RequestManager};
use crate::appdata::tabs::{TabItem, TabItemManager, TabType};
use crate::ui::http_method_badge::HttpMethodBadge;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};
use rustle_ui_components::context_menu::{ContextMenu, MenuItem};

#[component]
pub fn RequestToggleBar() -> Element {
    let mut tabs = TabItemManager::inject();
    let mut request_manager_signal = RequestManager::inject();

    let requests_to_display = request_manager_signal.read().items.clone();
    let bar_open = use_signal(|| true);

    return rsx! {
        toggle_bar::ToggleBar {
            title: "Requests".to_string(),
            class: "flex-grow bg-bg-secondary",
            open: bar_open,
            can_toggle: false,
            icon: rsx! {
                Icon {
                    icon: ld_icons::LdWifi,
                    width: 14,
                    height: 14,
                }
            },
            add_button: rsx! {
                button {
                    class: "hover:bg-item-hover-bg rounded-md",
                    onclick: move |_| {
                        request_manager_signal.with_mut(|manager| {
                            manager.insert_new();
                        })
                    },
                    Icon {
                        icon: ld_icons::LdPlus,
                        width: 16,
                        height: 16,
                    }
                }
            },
            body: rsx! {
                div {
                    for request in requests_to_display {
                        RequestItemContextMenu { request }
                    }
                }
            },
        }
    };
}

#[component]
fn RequestItemContextMenu(request: RequestItem) -> Element {
    let mut tabs = TabItemManager::inject();
    let mut manager = RequestManager::inject();

    let request_id = request.id.clone();

    return rsx! {
        ContextMenu {
            items: vec![
                MenuItem {
                    label: "Delete".to_string(),
                    on_click: Callback::new(move |_| {
                        manager.with_mut(|m| {
                            m.delete(request_id.clone());
                        })
                    }),
                    disabled: false,
                    icon: None,
                },
            ],
            button {
                class: "w-full flex items-center gap-2 pl-4 pr-2 py-0.5 hover:bg-item-hover-bg",
                onclick: move |_| {
                    let tman = &mut tabs.write();
                    let tab = TabItem::new(
                        request.name.clone(),
                        TabType::Request,
                        Some(request.id.clone()),
                    );

                    tman.add_tab(tab);
                },
                HttpMethodBadge {
                    method: request.method,
                }
                p {
                    class: "text-start text-nowrap truncate",
                    "{request.name}"
                }
            }
        }
    };
}

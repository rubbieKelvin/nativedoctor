use super::toggle_bar;
use crate::appdata::prelude::RequestItem;
use crate::appdata::tabs::{TabItem, TabItemManager, TabType};
use crate::ui::http_method_badge::HttpMethodBadge;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

#[component]
pub fn RequestToggleBar() -> Element {
    let requests = use_context::<Signal<Vec<RequestItem>>>();
    let mut tabs = use_context::<Signal<TabItemManager>>();
    let open = use_signal(|| true);

    return rsx! {
        toggle_bar::ToggleBar {
            title: "Requests".to_string(),
            class: (if open() { "flex-grow" } else { "" }).to_string(),
            open: open,
            add_button: rsx! {
                button {
                    class: "hover:bg-item-hover-bg rounded-md",
                    Icon {
                        icon: ld_icons::LdPlus,
                        width: 16,
                        height: 16,
                    }
                }
            },
            body: rsx! {
                div {
                    for request in requests() {
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
                                class: " ",
                                "{request.name}"
                            }
                        }
                    }
                }
            },
        }
    };
}

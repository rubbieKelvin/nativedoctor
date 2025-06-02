use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

use crate::appdata::tabs::{TabItem, TabItemManager};

#[component]
pub fn TabItemManagerUi(children: Element) -> Element {
    let tabs_manager_signal = TabItemManager::inject();
    let tabs_manager = tabs_manager_signal();

    return rsx! {
        div {
            class: "flex px-1",
            class: if tabs_manager_signal().tabs.is_empty() {
                "h-10"
            } else {
                "border-b"
            },
            for tab in tabs_manager.tabs {
                TabItemUi { item: tab }
            }
            {children}
        }
    };
}

#[component]
pub fn TabItemUi(item: TabItem) -> Element {
    let mut tab_manager = TabItemManager::inject();
    let is_current_tab = tab_manager().get_current_tab().is_some()
        && tab_manager().get_current_tab().unwrap().id == item.id;

    let border_class = if is_current_tab {
        "border-b-accent!"
    } else {
        "border-b-transparent!"
    };

    // i just had to do this 
    let cloned_item = item.clone();

    return rsx! {
        button {
            class: "p-2 border-b-2 {border_class} flex items-center justify-between gap-2 group",
            onclick: move |_| {
                let tman = &mut tab_manager.write();
                tman.set_current_tab(cloned_item.id.clone());
            },
            span {
                class: "text-sm",
                "{item.name}"
            }
            button {
                onclick: move |event| {
                    event.stop_propagation();
                    let tman = &mut tab_manager.write();
                    tman.remove_tab(item.id.clone());
                },
                Icon {
                    icon: ld_icons::LdX,
                    width: 10,
                    height: 10,
                    class: "invisible group-hover:visible"
                }
            }
        }
    };
}

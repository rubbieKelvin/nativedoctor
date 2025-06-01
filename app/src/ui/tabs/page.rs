use crate::{
    appdata::{
        requests::RequestManager,
        tabs::{TabItemManager, TabType},
    },
    ui::tabs::request::RequestPage,
};
use dioxus::prelude::*;

#[component]
pub fn TabPage() -> Element {
    let tab_manager = use_context::<Signal<TabItemManager>>();
    let requests = RequestManager::get_request_items();

    let no_page = rsx! {
        div {
            class: "flex items-center justify-center h-full",
            "No page for selected tab"
        }
    };

    let tabmanager_instance = tab_manager();
    let current_tab = match tabmanager_instance.get_current_tab() {
        Some(tab) => tab,
        None => return no_page,
    };

    let payload = match current_tab.payload {
        Some(payload) => payload,
        None => "".to_string(),
    };

    return rsx! {
        div {
            class: "flex-grow h-0 overflow-y-auto",

            match current_tab.tab_type {
                TabType::Request => {
                    let req = requests.iter().find(|r| r.id == payload);

                    match req {
                        Some(req) => {
                            rsx! {
                                RequestPage { request: req.clone() }
                            }
                        }
                        None => {
                            no_page
                        }
                    }
                }
                _ => {
                    no_page
                }
            }
        }
    };
}

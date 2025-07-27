use components_lib::{
    border::{Border, BorderStyleVariant},
    label::Label,
    pane::{Pane, PaneStyleVariant},
    tabs::{TabItemData, TabSet},
};
use dioxus::prelude::*;

use crate::{session::Session, views::project::utils::WorkspaceTab};

#[component]
pub fn CallSideBarList(tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let session = use_context::<Signal<Session>>();
    let calls = use_memo(move || session().get_calls());

    return rsx! {
        div {
            for call in calls() {
                CallItem {
                    call,
                    tabs
                }
            }
        }
    };
}

#[component]
fn CallItem(call: (String, Vec<String>), tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    return rsx! {
        Pane {
            class: "flex gap-2 px-2 items-center group/requestitem hover:bg-[#202020] py-1",
            style: PaneStyleVariant::Transparent,
            border: Border::bottom().with_style(BorderStyleVariant::Mild),
            onclick: {
                let call = call.clone();
                move |_| {
                    let mut tabs = tabs.write();
                    let tabdata = TabItemData::new(WorkspaceTab::Call(call.0.clone()));
                    let similar = tabs.get_similar(&tabdata).cloned();

                    // if this call is open somewhere, let's select it instead of opening a new one
                    if let Some(tabdata) = similar {
                        tabs.select(Some(tabdata.id));
                        return;
                    }

                    let id = tabdata.id.clone();
                    tabs.add_tab(tabdata);
                    tabs.select(Some(id));
                }
            },
            Label { class: "flex-grow", "{call.0}" }
        }
    };
}

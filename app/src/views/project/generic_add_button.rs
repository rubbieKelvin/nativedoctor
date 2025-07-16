use components_lib::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    tabs::{TabItemData, TabSet},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdPlus, Icon};

use crate::{
    session::{EnvironmentDefination, Session},
    views::project::{utils::WorkspaceTab, SideBarList},
};

#[component]
pub fn GenericAddButtonForSideBar(
    tab: SideBarList,
    session: Signal<Session>,
    tabset: Signal<TabSet<WorkspaceTab>>,
) -> Element {
    // ...
    let mut create_request = move || {
        let mut session = session.write();
        let created_defination = session.new_empty_request();
        let mut tabset = tabset.write();
        let tabitem = TabItemData::new(WorkspaceTab::Request(created_defination));
        tabset.add_tab(tabitem.clone());
        tabset.select(Some(tabitem.id));
    };

    let create_call = move || {
        tracing::warn!("Called create call. Not implemented yet");
    };

    let mut create_environment = move || {
        let env = EnvironmentDefination::new("Untitled environment".to_string());
        let mut tabset = tabset.write();
        let tabitem = TabItemData::new(WorkspaceTab::Environment(env));

        tabset.add_tab(tabitem.clone());
        tabset.select(Some(tabitem.id));
    };

    // title (tooltip)
    let tooltip_message = match tab {
        SideBarList::Calls => "Create call",
        SideBarList::Environments => "Create environment",
        SideBarList::Requests => "Create request"
    };

    return rsx! {
        Button {
            size: ButtonSizeVariant::Icon,
            style: ButtonStyleVariant::Ghost,
            title: tooltip_message,
            onclick: move |_| {
                match tab {
                    SideBarList::Calls => create_call(),
                    SideBarList::Requests => create_request(),
                    SideBarList::Environments => create_environment(),
                }
            },
            Icon { width: 14, height: 14, icon: LdPlus }
        }
    };
}

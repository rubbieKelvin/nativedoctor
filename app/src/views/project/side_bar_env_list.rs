use components_lib::prelude::{
    Border, BorderStyleVariant, Label, Pane, PaneStyleVariant, TabItemData, TabSet,
};
use dioxus::prelude::*;

use crate::{
    session::{EnvironmentDefination, Session},
    views::project::utils::WorkspaceTab,
};

#[component]
pub fn EnvSideBarList(tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let session = use_context::<Signal<Session>>();

    let environments = use_memo(move || session().get_environments());

    return rsx! {
        div {
            for env in environments() {
                EnvItem{env, tabs}
            }
        }
    };
}

#[component]
fn EnvItem(env: EnvironmentDefination, tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    return rsx! {
        Pane {
            class: "flex gap-2 px-2 items-center group/requestitem hover:bg-[#202020] py-1",
            style: PaneStyleVariant::Transparent,
            border: Border::bottom().with_style(BorderStyleVariant::Mild),
            onclick: {
                let env = env.clone();
                move |_| {
                    let mut tabs = tabs.write();
                    let tabdata = TabItemData::new(WorkspaceTab::Environment(env.clone()));
                    let similar = tabs.get_similar(&tabdata).cloned();

                    // if this env is open somewhere, let's select it instead of opening a new one
                    if let Some(tabdata) = similar {
                        tabs.select(Some(tabdata.id));
                        return;
                    }

                    let id = tabdata.id.clone();
                    tabs.add_tab(tabdata);
                    tabs.select(Some(id));
                }
            },
            Label { class: "flex-grow", "{env.name}" }
        }
    };
}

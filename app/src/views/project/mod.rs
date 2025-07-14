use crate::{
    session::{EnvironmentDefination, Session},
    views::project::utils::WorkspaceTab,
};
use components_lib::{
    border::Border,
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    buttongroup::{ButtonGroup, ButtonGroupInner},
    label::{Label, LabelStyleVariant},
    pane::{Pane, PaneStyleVariant},
    select::Select,
    tabs::{TabItemData, TabSet, TabState, TabsManager},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdPencil, LdPlus},
    Icon,
};
use project_info_tab::ProjectInfoTab;
use request_list::RequestList;
use request_tab::RequestPage;
use strum::IntoEnumIterator;
use welcome_tab::WelcomePage;

mod project_info_tab;
mod request_list;
mod request_tab;
mod utils;
mod welcome_tab;
use crate::components::WmDragArea;

#[component]
pub fn ProjectView(session: Session) -> Element {
    let session = use_context_provider(|| Signal::new(session));
    let mut opentabs: Signal<TabSet<WorkspaceTab>> = use_signal(|| TabSet::new(vec![]));

    // create the welcome tabs
    use_hook(move || {
        let mut opentabs = opentabs.write();
        let project: WorkspaceTab = session().into();
        let project_tabdata = TabItemData::new(project);
        let welcome_tabdata = TabItemData::new(WorkspaceTab::Welcome).set_closable(false);

        opentabs.select(Some(project_tabdata.id.clone()));
        opentabs.add_tabs(vec![welcome_tabdata, project_tabdata]);
    });

    return rsx! {
        div { class: "h-full flex",
            SideBar { tabs: opentabs }
            Workspace { tabs: opentabs }
        }
    };
}

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display)]
enum SideBarList {
    Requests,
    Calls,
}

impl ButtonGroupInner for SideBarList {
    fn render(&self) -> Element {
        return rsx! {
            Label { "{self}" }
        };
    }
}

// Manages the current ui state of the requests list
#[component]
fn SideBar(tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let mut session = use_context::<Signal<Session>>();

    let mut current_list = use_signal(|| SideBarList::Requests);
    let environments = use_memo(move || session().get_environments());

    // this should be part of open project session
    let selected_env = use_signal(|| {
        let environments = environments();
        if environments.len() > 0 {
            return Some(environments[0].clone());
        }
        return None;
    });

    // if seleted env changes, let's update session.current_env
    use_effect(move || {
        let mut session = session.write();
        let selected_env = selected_env();
        session.current_env = selected_env.map(|e| e.ref_id);
    });

    return rsx! {
        Pane {
            style: PaneStyleVariant::Darker,
            border: Border::right(),
            class: "w-[300px] h-full flex flex-col relative",
            WmDragArea { class: "h-8 w-full items-center absolute" }

            // name and version
            div { class: "pl-18 pt-1",
                Label { style: LabelStyleVariant::Ghost, "{session().name}" }
            }

            // env selector and buttons
            div { class: "flex px-2 py-2 items-center gap-2",

                Select::<EnvironmentDefination> {
                    class: "w-full h-full",
                    wrapper_class: "flex-grow",
                    value: selected_env,
                    placeholder: "--No env--",
                    items: environments(),
                }
                Button {
                    size: ButtonSizeVariant::Icon,
                    style: ButtonStyleVariant::Ghost,
                    onclick: move |_| {
                        let mut tabs = tabs.write();
                        let tabdata: TabItemData<WorkspaceTab> = TabItemData::new(session().into());
                        let similar = tabs.get_similar(&tabdata).cloned();
                        if let Some(tabdata) = similar {
                            tabs.select(Some(tabdata.id));
                            return;
                        }
                        let id = tabdata.id.clone();
                        tabs.add_tab(tabdata);
                        tabs.select(Some(id));
                    },
                    Icon { width: 14, height: 14, icon: LdPencil }
                }
                Pane {
                    class: "h-full",
                    style: PaneStyleVariant::Transparent,
                    border: Border::right(),
                }
                Button {
                    size: ButtonSizeVariant::Icon,
                    style: ButtonStyleVariant::Ghost,
                    onclick: move |_| {
                        let mut session = session.write();
                        let created_defination = session.new_empty_request();
                        let mut tabset = tabs.write();
                        let tabitem = TabItemData::new(WorkspaceTab::Request(created_defination));
                        tabset.add_tab(tabitem.clone());
                        tabset.select(Some(tabitem.id));
                    },
                    Icon { width: 14, height: 14, icon: LdPlus }
                }
            }

            // list tab
            div { class: "px-2 mb-2",
                ButtonGroup::<SideBarList> {
                    class: "flex gap-2",
                    child_class: "flex-[50%]",
                    value: current_list(),
                    buttons: SideBarList::iter().collect::<Vec<SideBarList>>(),
                    inactive_style: ButtonStyleVariant::Ghost,
                    onselect: move |v| {
                        current_list.set(v);
                    },
                }
            }

            Pane {
                class: "w-full",
                style: PaneStyleVariant::Transparent,
                border: Border::bottom(),
            }

            // requests
            div { class: "flex-grow h-0 overflow-y-auto",

                if current_list() == SideBarList::Requests {
                    RequestList { tabs }
                } else {
                    div { "Calls" }
                }
            }
        }
    };
}

#[component]
fn Workspace(tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    return rsx! {
        style {
            "
            #workspace-tab_tablist{{
                align-items: center;
                overflow-x: auto;
                white-space: nowrap;
                scrollbar-width: none;
                -ms-overflow-style: none;
            }}

            #workspace-tab_tablist::-webkit-scrollbar {{
                display: none;
            }}

            #workspace-tab_tablist::before,
            #workspace-tab_tablist::after {{
                content: "
            ";
                position: absolute;
                top: 0;
                bottom: 0;
                width: 50px;
                pointer-events: none;
                transition: opacity 0.2s ease-in-out;
            }}

            /* Left shadow */
            #workspace-tab_tablist::before {{
                left: 0;
                background: linear-gradient(to right, #1E1E1E, transparent);
            }}

            /* Right shadow */
            #workspace-tab_tablist::after {{
                right: 0;
                background: linear-gradient(to left, #1E1E1E, transparent);
            }}

            /* TODO: Fix; doesn't work yet */
            /* classes to control visibility from JavaScript */
            #workspace-tab_tablist.show-start-shadow::before,
            #workspace-tab_tablist.show-end-shadow::after {{
                opacity: 1;
            }}

            #workspace-tab_tablist::before,
            #workspace-tab_tablist::after {{
                opacity: 0;
            }}
            "
        }

        Pane { class: "flex-grow w-0 relative", style: PaneStyleVariant::Dark,
            TabsManager::<WorkspaceTab> {
                id: "workspace-tab",
                tabs,
                class: "p-2 h-full workspace-tab-wrapper",
                list_class: "overflow-y-auto",
                tab_real_estate: rsx! {
                    WmDragArea { class: "flex-grow h-full" }
                },
                TabContent {}
            }
        }
    };
}

#[component]
fn TabContent() -> Element {
    let state = use_context::<TabState<WorkspaceTab>>();
    return match state.tab.payload {
        WorkspaceTab::Welcome => rsx! {
            WelcomePage {}
        },
        WorkspaceTab::Request(_) => rsx! {
            RequestPage {}
        },
        WorkspaceTab::Project(name, description) => rsx! {
            ProjectInfoTab { name, description }
        }, // _ => rsx! {},
    };
}

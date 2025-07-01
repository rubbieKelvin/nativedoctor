use crate::{
    session::{RequestDefination, Session},
    views::project::utils::{get_label_style_for_method, WorkspaceTab},
};
use components_lib::{
    border::{Border, BorderStyleVariant},
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    buttongroup::{ButtonGroup, GroupButton},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    pane::{Pane, PaneStyleVariant},
    select::Select,
    tabs::{TabItemData, TabSet, TabState, TabsManager},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdEllipsisVertical, LdGithub, LdMail, LdPencil, LdPlus, LdTwitter},
    Icon,
};
use request_tab::RequestPage;

mod request_tab;
mod utils;
use crate::components::WmDragArea;

#[component]
pub fn ProjectView(session: Session) -> Element {
    use_context_provider(|| Signal::new(session));
    let mut opentabs: Signal<TabSet<WorkspaceTab>> = use_signal(|| TabSet::new(vec![]));

    // create the welcome tabs
    use_hook(|| {
        let mut opentabs = opentabs.write();
        let tabdata = TabItemData::new(WorkspaceTab::Welcome).set_closable(false);
        let id = tabdata.id.clone();

        opentabs.add_tab(tabdata);
        opentabs.select(Some(id));
    });

    return rsx! {
        div { class: "h-full flex",
            SideBar { tabs: opentabs }
            Workspace { tabs: opentabs }
        }
    };
}

#[derive(PartialEq, Clone)]
enum SideBarList {
    Requests,
    Calls,
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
        session.current_env = selected_env;
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

                Select::<String> {
                    class: "w-full h-full",
                    wrapper_class: "flex-grow",
                    value: selected_env,
                    placeholder: "--No env--",
                    items: environments(),
                }
                Button {
                    size: ButtonSizeVariant::Icon,
                    style: ButtonStyleVariant::Ghost,
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
                ButtonGroup {
                    class: "flex gap-2",
                    inactive_style: ButtonStyleVariant::Ghost,
                    GroupButton {
                        class: "flex-[50%]",
                        onclick: move |_| {
                            current_list.set(SideBarList::Requests);
                        },
                        Label { "Requests" }
                    }
                    GroupButton {
                        class: "flex-[50%]",
                        onclick: move |_| {
                            current_list.set(SideBarList::Calls);
                        },
                        Label { "Calls" }
                    }
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
fn RequestList(class: Option<String>, tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let session = use_context::<Signal<Session>>();

    return rsx! {
        div { class,
            for request in session().requests {
                RequestListItem { tabs, request }
            }
        }
    };
}

#[component]
fn RequestListItem(request: RequestDefination, tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let method_style = get_label_style_for_method(&request.method);

    return rsx! {
        Pane {
            class: "flex gap-2 px-2 items-center group/requestitem hover:bg-[#202020] py-1",
            style: PaneStyleVariant::Transparent,
            border: Border::bottom().with_style(BorderStyleVariant::Mild),
            onclick: {
                let request = request.clone();
                move |_| {
                    let mut tabs = tabs.write();
                    let tabdata = TabItemData::new(WorkspaceTab::Request(request.clone()));
                    let similar = tabs.get_similar(&tabdata).cloned();
                    if let Some(tabdata) = similar {
                        tabs.select(Some(tabdata.id));
                        return;
                    }
                    let id = tabdata.id.clone();
                    tabs.add_tab(tabdata);
                    tabs.select(Some(id));
                }
            },
            Label {
                class: "uppercase w-10",
                size: LabelSizeVariant::Small,
                style: method_style,
                "{request.method}"
            }
            Label { class: "flex-grow", "{request.name}" }
            Button {
                class: "opacity-0 group-hover/requestitem:opacity-100",
                style: ButtonStyleVariant::Transparent,
                size: ButtonSizeVariant::Icon,
                Icon {
                    width: 16,
                    height: 16,
                    class: "text-white",
                    icon: LdEllipsisVertical,
                }
            }
        }
    };
}

#[component]
fn Workspace(tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    return rsx! {
        Pane { class: "flex-grow", style: PaneStyleVariant::Dark,
            TabsManager::<WorkspaceTab> { tabs, class: "p-2 h-full", TabContent {} }
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
            RequestPage { }
        },
        _ => rsx! {},
    };
}

#[component]
fn WelcomePage() -> Element {
    let state = use_context::<TabState<WorkspaceTab>>();
    let mut tabset = state.tabs;
    let mut session = use_context::<Signal<Session>>();

    return rsx! {
        div { class: "h-full flex items-center justify-center",
            div { class: "flex flex-col gap-4 min-w-[40%]",

                div { class: "flex items-center",
                    div { class: "flex-grow",
                        Label {
                            size: LabelSizeVariant::Large,
                            style: LabelStyleVariant::Mild,
                            "Native Doctor"
                        }

                        Label {
                            size: LabelSizeVariant::Small,
                            style: LabelStyleVariant::Ghost,
                            "Let's f*#ing go!"
                        }
                    }

                    div { class: "flex gap-4 items-center",
                        Button {
                            size: ButtonSizeVariant::Icon,
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            Icon { icon: LdTwitter, height: 14, width: 14 }
                        }

                        Button {
                            size: ButtonSizeVariant::Icon,
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            Icon { icon: LdGithub, height: 14, width: 14 }
                        }

                        Button {
                            size: ButtonSizeVariant::Icon,
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            Icon { icon: LdMail, height: 14, width: 14 }
                        }
                    }
                }

                // buttons
                Pane {
                    class: "rounded-md flex flex-col p-2 gap-2",
                    border: Border::all(),

                    Button {
                        style: ButtonStyleVariant::Ghost,
                        class: "flex gap-2 items-center",
                        onclick: move |_| {
                            let mut session = session.write();
                            let created_defination = session.new_empty_request();

                            let mut tabset = tabset.write();
                            let tabitem = TabItemData::new(WorkspaceTab::Request(created_defination));
                            
                            tabset.add_tab(tabitem.clone());
                            tabset.select(Some(tabitem.id));
                        },
                        Icon { icon: LdPlus, height: 14, width: 14 }
                        Label { "Add a request" }
                    }
                    Button {
                        style: ButtonStyleVariant::Ghost,
                        class: "flex gap-2 items-center",
                        Icon { icon: LdPencil, height: 14, width: 14 }
                        Label { "Edit environment" }
                    }
                }
            }
        }
    };
}

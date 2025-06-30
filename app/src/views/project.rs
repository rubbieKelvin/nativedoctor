use crate::session::{RequestDefination, Session};
use components_lib::{
    border::{Border, BorderStyleVariant},
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    buttongroup::{ButtonGroup, GroupButton},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    pane::{Pane, PaneStyleVariant},
    select::Select,
    tabs::{TabItemData, TabPayload, TabSet, TabState, TabsManager},
};
use dioxus::prelude::*;

use dioxus_desktop::wry::http::method;
use dioxus_free_icons::{
    icons::ld_icons::{LdEllipsisVertical, LdGithub, LdHome, LdMail, LdPencil, LdPlus, LdTwitter},
    Icon,
};

use crate::components::WmDragArea;

fn get_label_style_for_method<S: AsRef<str>>(method: S) -> LabelStyleVariant {
    let method = method.as_ref();
    let method = method.to_lowercase();
    return match method.as_str() {
        "get" => LabelStyleVariant::Success,
        "post" => LabelStyleVariant::Info,
        "patch" => LabelStyleVariant::Warning,
        "delete" => LabelStyleVariant::Danger,
        "put" => LabelStyleVariant::Debug,
        _ => LabelStyleVariant::Mild,
    };
}

#[derive(PartialEq, Clone)]
enum WorkspaceTab {
    Welcome,
    Request(RequestDefination),
}

#[derive(PartialEq, Clone)]
enum WorkspaceTabId {
    Welcome,
    Request(uuid::Uuid),
}

impl TabPayload for WorkspaceTab {
    type Identifier = WorkspaceTabId;
    fn render_title(&self, selected: bool) -> Element {
        return match self {
            WorkspaceTab::Welcome => rsx! {
                div { class: "flex gap-1 items-center",
                    Icon { icon: LdHome, height: 12, width: 12 }
                    Label {
                        class: "flex-grow text-start",
                        style: LabelStyleVariant::Mild,
                        "Home"
                    }
                }
            },
            WorkspaceTab::Request(request) => rsx! {
                div { class: "flex gap-1 items-center",
                    Label {
                        class: "uppercase",
                        size: LabelSizeVariant::Tiny,
                        style: get_label_style_for_method(&request.method),
                        "{request.method}"
                    }
                    Label {
                        class: "flex-grow",
                        style: if selected { LabelStyleVariant::Default } else { LabelStyleVariant::Mild },
                        "{request.name}"
                    }
                }
            },
        };
    }

    fn unique_identifier(&self) -> Self::Identifier {
        return match self {
            WorkspaceTab::Welcome => WorkspaceTabId::Welcome,
            WorkspaceTab::Request(request) => WorkspaceTabId::Request(request.id),
        };
    }
}

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
    // let project_state = ProjectState::inject();
    // let open_request_signal = project_state.open_request.clone();
    // let project_signal = project_state.project.clone();
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
                        session.new_empty_request();
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
            WelcomePage { state }
        },
        _ => rsx! {},
    };
}

#[component]
fn WelcomePage(state: TabState<WorkspaceTab>) -> Element {
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
                            "Let's get started"
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

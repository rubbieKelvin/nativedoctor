use components_lib::{
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    tabs::TabPayload,
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdBox, LdHome, LdLayers, LdNetwork},
    Icon,
};

use crate::{
    session::{EnvironmentDefination, RequestDefination, Session},
    views::project::{
        tab_page_call::CallPage, tab_page_env::EnvPage, tab_page_project_info::ProjectInfoTab,
        tab_page_request::RequestPage, welcome_tab::WelcomePage,
    },
};

pub fn get_label_style_for_method<S: AsRef<str>>(method: S) -> LabelStyleVariant {
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

// defines the possible workspace tab (can include the kind of data they carry)
#[derive(PartialEq, Clone)]
pub enum WorkspaceTab {
    Welcome,
    Call(String),
    Project(String, String),
    Request(RequestDefination),
    Environment(EnvironmentDefination),
}

// defines how workspace tabs can be identified
#[derive(PartialEq, Clone)]
pub enum WorkspaceTabId {
    Welcome,
    Project,
    Call(String),
    Request(uuid::Uuid),
    Environment(uuid::Uuid),
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
            WorkspaceTab::Project(..) => rsx! {
                div { class: "flex gap-1 items-center",
                    Icon { icon: LdBox, height: 12, width: 12 }
                    Label {
                        class: "flex-grow text-start",
                        style: LabelStyleVariant::Mild,
                        "Project"
                    }
                }
            },
            WorkspaceTab::Request(request) => rsx! {
                div { class: "flex gap-1 items-center text-nowrap",
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
            WorkspaceTab::Environment(env) => rsx! {
                div { class: "flex gap-1 items-center",
                    Icon { icon: LdLayers, height: 12, width: 12 }
                    Label {
                        class: "flex-grow text-start",
                        style: LabelStyleVariant::Mild,
                        "{env.name}"
                    }
                }
            },
            WorkspaceTab::Call(call) => rsx! {
                div { class: "flex gap-1 items-center",
                    Icon { icon: LdNetwork, height: 12, width: 12 }
                    Label {
                        class: "flex-grow text-start",
                        style: LabelStyleVariant::Mild,
                        "{call}"
                    }
                }
            },
        };
    }

    fn render_content(&self) -> Element {
        return match self {
            WorkspaceTab::Welcome => rsx! {
                WelcomePage {}
            },
            WorkspaceTab::Request(_) => rsx! {
                RequestPage {}
            },
            WorkspaceTab::Project(name, description) => rsx! {
                ProjectInfoTab { name, description }
            },
            WorkspaceTab::Environment(env) => rsx! {
                EnvPage {env: env.clone()}
            },
            WorkspaceTab::Call(_) => rsx! {
                CallPage {  }
            },
        };
    }

    fn unique_identifier(&self) -> Self::Identifier {
        return match self {
            WorkspaceTab::Welcome => WorkspaceTabId::Welcome,
            WorkspaceTab::Project(..) => WorkspaceTabId::Project,
            WorkspaceTab::Request(request) => WorkspaceTabId::Request(request.id),
            WorkspaceTab::Environment(env) => WorkspaceTabId::Environment(env.id),
            WorkspaceTab::Call(call) => WorkspaceTabId::Call(call.clone()),
        };
    }
}

// create a workspace tab from a session
impl Into<WorkspaceTab> for Session {
    fn into(self) -> WorkspaceTab {
        WorkspaceTab::Project(self.name, self.description)
    }
}

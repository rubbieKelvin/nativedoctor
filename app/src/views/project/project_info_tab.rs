use std::collections::HashMap;

use components_lib::{
    tabs::TabState,
    textfield::{TextField, TextFieldSizeVariant, TextFieldStyleVariant},
};
use dioxus::prelude::*;

use crate::{session::Session, views::project::utils::WorkspaceTab};

#[component]
pub fn ProjectInfoTab(
    name: String,
    description: String,
    envs: HashMap<String, HashMap<String, String>>,
) -> Element {
    let tab_state = use_context::<TabState<WorkspaceTab>>();

    let mut tabset = tab_state.tabs;
    let mut session = use_context::<Signal<Session>>();

    let project_name = use_signal(|| name.clone());
    let project_description = use_signal(|| description);
    let project_envs = use_signal(|| envs);

    // the tab holds some data that the page might need when recreated
    // this is nessesary because when switching tabs, a new instance of the page is created
    // so i stored tab data in the WorkspaceTab enum, and i need to update it when we change some data,
    // so when the page is recreated, the page state is maintained
    let save_tab_data = move || {
        let mut tabset = tabset.write();
        if let Some(tab) = tabset.get_selected_mut() {
            tab.payload =
                WorkspaceTab::Project(project_name(), project_description(), project_envs());
        }
    };

    let on_project_name_updated = move || {
        // if new name is empty, use the old name
        let new_name = project_name();
        let new_name = new_name.trim();
        let new_name = if new_name.is_empty() {
            if name.is_empty() {
                "Untitled".to_string()
            } else {
                name.clone()
            }
        } else {
            new_name.to_string()
        };

        // update the session
        let mut session = session.write();
        session.name = new_name;
    };

    let on_project_description_updated = move || {
        let new_description = project_description();
        let new_description = new_description.trim();

        let mut session = session.write();
        session.description = new_description.to_string();
    };

    return rsx! {
        div { class: "h-full flex flex-col gap-4 pt-2",
            div { class: "flex flex-col gap-1",
                TextField {
                    placeholder: "Name",
                    value: project_name,
                    style: TextFieldStyleVariant::Ghost,
                    size: TextFieldSizeVariant::Large,
                    oninput: {
                        let mut save_tab_data = save_tab_data.clone();
                        move |_| {
                            save_tab_data();
                        }
                    },
                    onblur: {
                        let mut on_project_name_updated = on_project_name_updated.clone();
                        move |_| {
                            on_project_name_updated();
                        }
                    },
                    onreturn: {
                        let mut on_project_name_updated = on_project_name_updated.clone();
                        move |_| {
                            on_project_name_updated();
                        }
                    },
                }
                TextField {
                    placeholder: "Description",
                    value: project_description,
                    style: TextFieldStyleVariant::Ghost,
                    oninput: {
                        let mut save_tab_data = save_tab_data.clone();
                        move |_| {
                            save_tab_data();
                        }
                    },
                    onblur: {
                        let mut on_project_description_updated = on_project_description_updated.clone();
                        move |_| {
                            on_project_description_updated();
                        }
                    },
                    onreturn: {
                        let mut on_project_description_updated = on_project_description_updated.clone();
                        move |_| {
                            on_project_description_updated();
                        }
                    },
                }
            }
        }
    };
}

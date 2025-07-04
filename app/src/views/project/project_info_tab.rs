use std::collections::HashMap;

use components_lib::{
    border::Border,
    label::Label,
    pane::{Pane, PaneStyleVariant},
    tableinput::{CellValue, TableInput, TableInputCell},
    tabs::TabState,
    textfield::{TextField, TextFieldSizeVariant, TextFieldStyleVariant},
};
use dioxus::prelude::*;

use crate::{session::Session, views::project::utils::WorkspaceTab};

#[derive(Clone, PartialEq, strum::Display)]
enum EnvHeader {
    NameColumn,
    EnvColumn(String),
}

impl TableInputCell for EnvHeader {
    fn identifier(&self) -> String {
        return match self {
            EnvHeader::NameColumn => String::new(),
            EnvHeader::EnvColumn(s) => s.clone(),
        };
    }

    fn render_input(
        &self,
        value: CellValue,
        _row: HashMap<String, CellValue>,
        set: impl Fn(CellValue) + 'static,
        _set_partial: impl Fn(HashMap<String, CellValue>) + 'static,
    ) -> Element {
        rsx! {
            TextField {
                class: format!(
                    "w-48 {}",
                    match self {
                        EnvHeader::EnvColumn(n) => {
                            if n == "Default" {
                                "!bg-[#f2c43d]/10 !rounded-none".to_string()
                            } else {
                                "".to_string()
                            }
                        }
                        _ => "".to_string(),
                    },
                ),
                value: value.to_string(),
                placeholder: match self {
                    EnvHeader::NameColumn => "name",
                    EnvHeader::EnvColumn(_) => "value",
                },
                style: TextFieldStyleVariant::Ghost,
                oninput: move |e: Event<FormData>| {
                    let value = e.value();
                    if value.trim() == "" {
                        set(CellValue::Empty);
                    } else {
                        set(CellValue::Text(value.to_string()));
                    }
                },
            }
        }
    }
}

#[component]
pub fn ProjectInfoTab(
    name: String,
    description: String,
    envs: HashMap<String, HashMap<String, String>>,
) -> Element {
    let tab_state = use_context::<TabState<WorkspaceTab>>();

    let mut tabset = tab_state.tabs;
    let mut session = use_context::<Signal<Session>>();

    let mut project_name = use_signal(|| name.clone());
    let mut project_description = use_signal(|| description);
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

    let mut env: Signal<Vec<HashMap<String, CellValue>>> = use_signal(|| vec![]);
    let environments = use_memo(move || session().get_environments());
    let columns = use_memo(move || {
        vec![
            vec![EnvHeader::NameColumn],
            environments()
                .iter()
                .map(|e| EnvHeader::EnvColumn(e.clone()))
                .collect::<Vec<EnvHeader>>(),
        ]
        .concat()
    });

    return rsx! {
        div { class: "h-full flex flex-col gap-4 pt-2",
            div { class: "flex flex-col gap-1",
                TextField {
                    placeholder: "Name",
                    value: "{project_name}",
                    style: TextFieldStyleVariant::Ghost,
                    size: TextFieldSizeVariant::Large,
                    oninput: {
                        let mut save_tab_data = save_tab_data.clone();
                        move |e: Event<FormData>| {
                            project_name.set(e.value());
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
                    value: "{project_description}",
                    style: TextFieldStyleVariant::Ghost,
                    oninput: {
                        let mut save_tab_data = save_tab_data.clone();
                        move |e: Event<FormData>| {
                            project_description.set(e.value());
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

            div {
                Pane { class: "flex gap-1 border-b", border: Border::bottom(),
                    for col in columns() {
                        match col {
                            EnvHeader::NameColumn => rsx! {
                                Pane { class: "w-48", style: PaneStyleVariant::Dark }
                            },
                            EnvHeader::EnvColumn(s) => {
                                if s == "Default" {
                                    rsx! {
                                        Label { class: "w-48 !bg-[#f2c43d]/10 p-2", "{s}" }
                                    }
                                } else {
                                    rsx! {
                                        
                                        Label { class: "w-48 p-2", "{s}" }
                                    }
                                }
                            }
                        }
                    }
                }

                TableInput {
                    value: env(),
                    row_class: "gap-1 flex",
                    columns: columns(),
                    onchange: move |rows| {
                        env.set(rows);
                    },
                }
            }
        }
    };
}

use components_lib::{
    tabs::TabState,
    textfield::{TextField, TextFieldSizeVariant, TextFieldStyleVariant},
};
use dioxus::prelude::*;

use crate::{session::Session, views::project::utils::WorkspaceTab};

#[component]
pub fn ProjectInfoTab(name: String, description: String) -> Element {
    let tab_state = use_context::<TabState<WorkspaceTab>>();

    let mut tabset = tab_state.tabs;
    let mut session = use_context::<Signal<Session>>();

    let mut project_name = use_signal(|| name.clone());
    let mut project_description = use_signal(|| description);

    // the tab holds some data that the page might need when recreated
    // this is nessesary because when switching tabs, a new instance of the page is created
    // so i stored tab data in the WorkspaceTab enum, and i need to update it when we change some data,
    // so when the page is recreated, the page state is maintained
    let save_tab_data = move || {
        let mut tabset = tabset.write();
        if let Some(tab) = tabset.get_selected_mut() {
            tab.payload = WorkspaceTab::Project(project_name(), project_description());
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

            // div {
            //     Pane { class: "flex gap-1 border-b", border: Border::bottom(),
            //         for col in columns() {
            //             match col {
            //                 EnvHeader::NameColumn => rsx! {
            //                     Pane { class: "w-48", style: PaneStyleVariant::Dark }
            //                 },
            //                 EnvHeader::EnvColumn(s) => {
            //                     if s == "Default" {
            //                         rsx! {
            //                             Label { class: "w-48 !bg-[#f2c43d]/10 p-2", "{s}" }
            //                         }
            //                     } else {
            //                         rsx! {
            //                             EnvHeaderInput { value: "{s}", onaccepted: move |v: String| {} }
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     }

            //     TableInput {
            //         value: env(),
            //         row_class: "gap-1 flex",
            //         columns: columns(),
            //         onchange: move |rows| {
            //             env.set(rows);
            //         },
            //     }
            // }
        }
    };
}

#[component]
fn EnvHeaderInput(value: String, onaccepted: EventHandler<String>) -> Element {
    let mut reactive_value = use_signal(|| value.clone());

    return rsx! {
        TextField {
            value: "{reactive_value}",
            style: TextFieldStyleVariant::Void,
            class: "w-48",
            oninput: move |e: Event<FormData>| {
                let val = e.value();
                reactive_value.set(val);
            },
            onblur: {
                let value = value.clone();
                let mut reactive_value = reactive_value.clone();
                move |_| {
                    reactive_value.set(value.clone());
                }
            },
            onreturn: {
                let mut reactive_value = reactive_value.clone();
                move |_: Event<KeyboardData>| {
                    let v = reactive_value();
                    if v.len() == 0 || &v == "*" {
                        reactive_value.set(value.clone());
                    }
                    onaccepted.call(v);
                }
            },
        }
    };
}

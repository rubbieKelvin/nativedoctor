use std::collections::HashMap;

use components_lib::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::Label,
    prelude::{
        CellValue, TableInput, TableInputCell, TextField, TextFieldSizeVariant,
        TextFieldStyleVariant,
    },
    tabs::TabState,
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdSquare, LdSquareCheck},
    Icon,
};
use strum::IntoEnumIterator;

use crate::{session::EnvironmentDefination, views::project::utils::WorkspaceTab};

#[derive(PartialEq, Clone, strum::Display, strum::EnumIter)]
pub enum EnvTableColumn {
    Name,
    Sensitive,
    InitialValue,
    Value,
}

impl TableInputCell for EnvTableColumn {
    fn identifier(&self) -> String {
        return self.to_string();
    }

    fn internally_treat_as_empty(&self) -> bool {
        return !matches!(self, EnvTableColumn::Name);
    }

    fn render_input(
        &self,
        value: CellValue,
        _row: HashMap<String, CellValue>,
        set: impl Fn(CellValue) + 'static,
        _set_partial: impl Fn(HashMap<String, CellValue>) + 'static,
    ) -> Element {
        return match self {
            EnvTableColumn::Sensitive => rsx! {
                div { class: "w-22 flex items-center",
                    Button {
                        style: ButtonStyleVariant::Ghost,
                        size: ButtonSizeVariant::Icon,
                        onclick: {
                            let value = value.clone();
                            move |_| {
                                let value = value.clone();
                                set(CellValue::Boolean(!value.to_boolean().unwrap_or_default()))
                            }
                        },
                        if value.to_boolean().unwrap_or_default() {
                            Icon { height: 16, width: 16, icon: LdSquareCheck }
                        } else {
                            Icon { height: 16, width: 16, icon: LdSquare }
                        }
                    }
                }
            },
            EnvTableColumn::Value | EnvTableColumn::InitialValue => rsx! {
                TextField {
                    class: "flex-grow !rounded-none focus-within:bg-[#3e3e3e]/20",
                    style: TextFieldStyleVariant::Void,
                    value: value.to_string(),
                    placeholder: "Value",
                    oninput: move |e: Event<FormData>| {
                        let value = e.value();
                        let value = value.trim();
                        if value.is_empty() {
                            set(CellValue::Empty)
                        } else {
                            set(CellValue::Text(value.to_string()))
                        }
                    },
                }
            },
            _ => rsx! {
                TextField {
                    class: "w-48 !rounded-none focus-within:bg-[#3e3e3e]/20",
                    value: value.to_string(),
                    placeholder: "Name",
                    style: TextFieldStyleVariant::Void,
                    oninput: move |e: Event<FormData>| {
                        let value = e.value();
                        if value.trim() == "" {
                            set(CellValue::Empty);
                        } else {
                            set(CellValue::Text(value.to_string()));
                        }
                    },
                }
            },
        };
    }
}

#[component]
pub fn EnvPage(env: EnvironmentDefination) -> Element {
    // TODO: cleanup
    let env_page_id = use_memo(|| nanoid::nanoid!());

    let mut env_name = use_signal(|| env.name.clone());
    let mut env_table = use_signal::<Vec<HashMap<String, CellValue>>>(|| env.into_table_data());

    let tab_state = use_context::<TabState<WorkspaceTab>>();

    let tab = tab_state.tab.clone();
    let tab_set = tab_state.tabs.clone();

    let is_current_tab = use_memo(move || {
        let tab_set = tab_set();
        let selected_id = tab_set.get_selected_id();

        return match selected_id {
            Some(id) => id == tab.id,
            None => false,
        };
    });

    // Function to save env to project session and save project
    let save_env = move || {};

    // We need to give focus to this component when it loads
    // use_effect(|| {
    //     // we need to make sure this page is the active focus fallback when it's active in selected tab
    //     document::eval(format!("
    //         function ensureFallback(){{
    //             const fallback = document.getElementById('{env_page_id}')
    //         }}
    //     ").as_str());
    // });

    return rsx! {
        style {
            "
            .env-page-table > div:first-child {{
                border-top: 1px solid #3e3e3e;
            }}
            .env-page-table > div:not(:last-child) {{
                border-bottom: 1px solid #3e3e3e;
            }}
            "
        }
        div {
            id: env_page_id,
            class: "h-full flex pt-2 flex-col gap-4",
            tabindex: -1,
            TextField {
                placeholder: "Environment Name",
                value: "{env_name}",
                style: TextFieldStyleVariant::Ghost,
                size: TextFieldSizeVariant::Large,
                oninput: move |e: Event<FormData>| {
                    let value = e.value();
                    env_name.set(value);
                },
            }

            // table header
            div { class: "flex",
                Label { class: "w-48", "Name" }
                Label { class: "w-24", "Secret" }
                Label { class: "flex-grow", "Initial Value" }
                Label { class: "flex-grow ml-1", "Current Value" }
            }

            // table input
            TableInput {
                class: "env-page-table",
                value: env_table(),
                columns: EnvTableColumn::iter().collect(),
                onchange: move |new_value| {
                    let mut rows = env_table.write();
                    *rows = new_value;
                },
            }
        }
    };
}

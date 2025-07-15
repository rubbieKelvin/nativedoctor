use std::collections::HashMap;

use components_lib::{button::{Button, ButtonSizeVariant, ButtonStyleVariant}, prelude::{
    CellValue, TableInput, TableInputCell, TextField, TextFieldSizeVariant, TextFieldStyleVariant,
}};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSquare, Icon};
use strum::IntoEnumIterator;

#[derive(PartialEq, Clone, strum::Display, strum::EnumIter)]
enum EnvTableColumn {
    Name,
    Sensitive,
    InitialValue,
    Value,
}

impl TableInputCell for EnvTableColumn {
    fn identifier(&self) -> String {
        return self.to_string();
    }

    // fn internally_treat_as_empty(&self) -> bool {
    //     return !matches!(self, EnvTableColumn::Name);
    // }

    fn render_input(
        &self,
        value: CellValue,
        row: HashMap<String, CellValue>,
        set: impl Fn(CellValue) + 'static,
        set_partial: impl Fn(HashMap<String, CellValue>) + 'static,
    ) -> Element {
        return match self {
            EnvTableColumn::Sensitive => rsx! {
                Button {
                    style: ButtonStyleVariant::Ghost,
                    size: ButtonSizeVariant::Icon,
                    Icon { height: 16, width: 16, icon: LdSquare }
                }
            },
            EnvTableColumn::Value | EnvTableColumn::InitialValue => rsx! {
                TextField {
                    class: "flex-grow",
                    style: TextFieldStyleVariant::Ghost,
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
                    class: "w-48",
                    value: value.to_string(),
                    placeholder: "Name",
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
            },
        };
    }
}

#[component]
pub fn EnvPage() -> Element {
    let mut env_name = use_signal(|| String::new());
    let mut env_table = use_signal::<Vec<HashMap<String, CellValue>>>(|| vec![]);

    return rsx! {
        div { class: "h-full flex pt-2 flex-col",
            TextField {
                placeholder: "Name",
                value: "{env_name}",
                style: TextFieldStyleVariant::Ghost,
                size: TextFieldSizeVariant::Large,
                oninput: move |e: Event<FormData>| {
                    let value = e.value();
                    env_name.set(value);
                },
            }
            // table input
            TableInput {
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

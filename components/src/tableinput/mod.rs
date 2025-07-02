use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;

use crate::border::Border;

pub trait TableInputColumn {
    fn identifier(&self) -> String;
    fn render_header(&self) -> Element;
    fn render_input(&self) -> Element;
}

#[derive(PartialEq, Clone)]
pub enum ColumnValue {
    Text(String),
    Number(i64),
    Boolean(bool),
    Path(PathBuf),
}

#[component]
pub fn TableInput<T: TableInputColumn + PartialEq + Clone + 'static>(
    class: Option<String>,
    border: Option<Border>,
    columns: Vec<T>,
) -> Element {
    let border = border.unwrap_or_default();
    let class = format!("{} {}", class.unwrap_or_default(), border.classes());
    let rows: Signal<Vec<HashMap<String, ColumnValue>>> = use_signal(|| vec![]);

    return rsx! {
        table {
            class: class,
            thead {
                tr {
                    for column in columns {
                        th {
                            class: "font-normal",
                            {column.render_header()} }
                    }
                }
            }
            tbody {
                tr {
                    td { "Name" }
                    td { "Value" }
                }
                tr {
                    td { "Name" }
                    td { "Value" }
                }
            }
        }
    };
}

#[component]
fn TableInputRow<T: TableInputColumn + PartialEq + Clone + 'static>(columns: Vec<T>) -> Element {
    let value: Signal<HashMap<String, ColumnValue>> = use_signal(|| HashMap::new());

    return rsx! {
        tr {
            for column in columns {
                td {
                    {column.render_input()}
                }
            }
        }
    };
}
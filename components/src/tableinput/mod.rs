use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;

use crate::border::Border;

pub trait TableInputColumn {
    fn is_required(&self) -> bool {
        return false;
    }

    fn identifier(&self) -> String;
    fn render_header(&self) -> Element;
    fn render_input(&self) -> Element;
}

#[derive(PartialEq, Clone)]
pub enum ColumnValue {
    Empty,
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
    let mut rows: Signal<Vec<HashMap<String, ColumnValue>>> = use_signal(|| vec![]);
    let columns = use_context_provider::<Vec<T>>(|| columns);

    // Returns true if all the required columns are filled for each row
    let all_rows_are_satisfied = {
        let columns = columns.clone();
        
        use_memo(move || {
            rows().iter().all(|row| {
                row.iter().all(|(id, value)| {
                    let column = columns.iter().find(|c| c.identifier() == *id);
                    if column.is_none() {
                        tracing::error!("Column not found: {}, this should not happen", id);
                        return false;
                    }

                    let column = column.unwrap();
                    if column.is_required() && matches!(value, ColumnValue::Empty) {
                        return false;
                    }

                    return true;
                })
            })
        })
    };

    // if all rows are satisfied add a new row
    {
        let columns = columns.clone();

        use_effect(move || {
            if all_rows_are_satisfied() {
                let mut row: HashMap<String, ColumnValue> = HashMap::new();
                for column in columns.iter() {
                    row.insert(column.identifier(), ColumnValue::Empty);
                }

                let mut rows = rows.write();
                rows.push(row);
            }
        })
    };

    return rsx! {
        table {
            class: class,
            thead {
                tr {
                    for column in columns.iter() {
                        th {
                            class: "font-normal",
                            {column.render_header()} }
                    }
                }
            }
            tbody {
                for row in rows().iter() {
                    TableInputRow<T> { }
                }
            }
        }
    };
}

#[component]
fn TableInputRow<T: TableInputColumn + PartialEq + Clone + 'static>() -> Element {
    let columns = use_context::<Vec<T>>();
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

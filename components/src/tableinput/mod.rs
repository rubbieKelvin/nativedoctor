use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;

use crate::border::Border;

pub trait TableInputCell {
    fn is_required(&self) -> bool {
        return false;
    }

    fn identifier(&self) -> String;
    fn render_header(&self) -> Element;
    fn render_input(&self, value: CellValue, set: impl Fn(CellValue) + 'static) -> Element;
}

#[derive(PartialEq, Clone, Default)]
pub enum CellValue {
    #[default]
    Empty,
    Text(String),
    Number(i64),
    Boolean(bool),
    Path(PathBuf),
}

#[allow(unused)]
impl CellValue {
    pub fn to_string(&self) -> Option<String> {
        return match self {
            CellValue::Text(v) => Some(v.clone()),
            _ => None,
        };
    }
    pub fn to_i64(&self) -> Option<i64> {
        return match self {
            CellValue::Number(v) => Some(v.clone()),
            _ => None,
        };
    }
    pub fn to_boolean(&self) -> Option<bool> {
        return match self {
            CellValue::Boolean(v) => Some(v.clone()),
            _ => None,
        };
    }
    pub fn to_path_buf(&self) -> Option<PathBuf> {
        return match self {
            CellValue::Path(v) => Some(v.clone()),
            _ => None,
        };
    }
}

#[component]
pub fn TableInput<T: TableInputCell + PartialEq + Clone + 'static>(
    class: Option<String>,
    border: Option<Border>,
    columns: Vec<T>,
) -> Element {
    let border = border.unwrap_or_default();
    let class = format!("{} {}", class.unwrap_or_default(), border.classes());
    let mut rows: Signal<Vec<HashMap<String, CellValue>>> = use_signal(|| vec![]);
    let columns = use_context_provider::<Vec<T>>(|| columns);

    // Returns true if there is at least one non-empty column for each row
    let all_rows_are_satisfied = use_memo(move || {
        rows()
            .iter()
            .all(|row| row.values().any(|value| !matches!(value, CellValue::Empty)))
    });

    // if all rows are satisfied add a new row
    {
        let columns = columns.clone();

        use_effect(move || {
            if all_rows_are_satisfied() {
                let mut row: HashMap<String, CellValue> = HashMap::new();
                for column in columns.iter() {
                    row.insert(column.identifier(), CellValue::Empty);
                }

                let mut rows = rows.write();
                rows.push(row);
            }
        })
    };

    return rsx! {
        table { class,
            thead {
                tr {
                    for column in columns.iter() {
                        th { class: "font-normal", {column.render_header()} }
                    }
                }
            }
            tbody {
                for (index , row) in rows().iter().enumerate() {
                    TableInputRow::<T> {
                        value: row.clone(),
                        onupdate: move |new_row_value| {
                            let mut rows = rows.write();
                            if let Some(row) = rows.get_mut(index) {
                                *row = new_row_value;
                            }
                        },
                    }
                }
            }
        }
    };
}

#[component]
fn TableInputRow<T: TableInputCell + PartialEq + Clone + 'static>(
    value: HashMap<String, CellValue>,
    onupdate: EventHandler<HashMap<String, CellValue>>,
) -> Element {
    let columns = use_context::<Vec<T>>();
    let mut value = use_signal(|| value);

    return rsx! {
        tr {
            for column in columns {
                TableInputColumn::<T> {
                    column: column.clone(),
                    value: {
                        let value = value();
                        let column_value = value.get(&column.identifier());
                        let column_value = column_value.map(|v| v.clone());
                        column_value.unwrap_or_default()
                    },
                    onchange: {
                        move |new| {
                            let mut value = value.write();
                            value.insert(column.identifier().clone(), new);
                            onupdate.call(value.clone());
                        }
                    },
                }
            }
        }
    };
}

#[component]
fn TableInputColumn<T: TableInputCell + PartialEq + Clone + 'static>(
    column: T,
    value: CellValue,
    onchange: EventHandler<CellValue>,
) -> Element {
    return rsx! {
        td { {column.render_input(value, move |new_value| { onchange.call(new_value) })} }
    };
}

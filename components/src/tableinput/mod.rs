use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;
use strum::Display;

use crate::border::Border;

pub trait TableInputCell {
    fn is_required(&self) -> bool {
        return false;
    }

    fn identifier(&self) -> String;
    fn render_header(&self) -> Element;
    fn render_input(&self, value: CellValue, set: impl Fn(CellValue) + 'static) -> Element;
}

#[allow(unused)]
#[derive(PartialEq, Clone, Default, Display, Debug)]
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
    value: Vec<HashMap<String, CellValue>>,
    onchange: Option<EventHandler<Vec<HashMap<String, CellValue>>>>,
) -> Element {
    let border = border.unwrap_or_default();
    let class = format!("{} {}", class.unwrap_or_default(), border.classes());
    let mut rows = use_signal(|| value.clone());
    let columns = use_context_provider::<Vec<T>>(|| columns);

    // Returns true if there is at least one non-empty column for each row
    // meaning in every row, one column has a value in it
    let all_rows_are_satisfied = use_memo(move || {
        rows()
            .iter()
            .all(|row| row.values().any(|value| !matches!(value, CellValue::Empty)))
    });

    let empty_row_indexes = use_memo(move || {
        let indeces = rows()
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                if row.values().all(|cell| matches!(cell, CellValue::Empty)) {
                    return Some(index);
                }
                return None;
            })
            .collect::<Vec<usize>>();
        return indeces;
    });

    {
        let columns = columns.clone();

        use_effect(move || {
            // if all rows are satisfied add a new row
            if all_rows_are_satisfied() {
                let mut row: HashMap<String, CellValue> = HashMap::new();
                for column in columns.iter() {
                    row.insert(column.identifier(), CellValue::Empty);
                }

                let mut rows = rows.write();
                rows.push(row);
                if let Some(onchange) = onchange {
                    onchange.call(rows.clone());
                }
            }

            // if there are more than one empty row, clean up
            let empty_row_indexes = empty_row_indexes();
            if empty_row_indexes.len() > 1 {
                let slice = &empty_row_indexes[0..empty_row_indexes.len() - 1];
                let new_row = rows()
                    .iter()
                    .enumerate()
                    .filter_map(|(index, row)| {
                        if slice.contains(&index) {
                            return None;
                        }
                        return Some(row.clone());
                    })
                    .collect::<Vec<HashMap<String, CellValue>>>();
                rows.set(new_row.clone());
                if let Some(onchange) = onchange {
                    onchange.call(new_row);
                }
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
                    InputRow::<T> {
                        value: row.clone(),
                        onchange: move |new_row_value| {
                            let mut rows = rows.write();
                            if let Some(row) = rows.get_mut(index) {
                                *row = new_row_value;
                            }
                            if let Some(onchange) = onchange {
                                onchange.call(rows.clone());
                            }
                        },
                    }
                }
            }
        }
    };
}

#[derive(Props, PartialEq, Clone)]
struct InputRowProps {
    value: HashMap<String, CellValue>,
    onchange: EventHandler<HashMap<String, CellValue>>,
}

#[component]
fn InputRow<T: TableInputCell + PartialEq + Clone + 'static>(props: InputRowProps) -> Element {
    let columns = use_context::<Vec<T>>();
    let mut row_value: Signal<HashMap<String, CellValue>> = use_signal(|| props.value);

    return rsx! {
        tr {
            for column in columns {
                TableInputColumn::<T> {
                    column: column.clone(),
                    value: {
                        let current_row = row_value();
                        let column_value = current_row.get(&column.identifier());
                        let column_value = column_value.map(|v| v.clone());
                        column_value.unwrap_or_default()
                    },
                    onchange: {
                        move |new| {
                            let mut current_row = row_value.write();
                            current_row.insert(column.identifier().clone(), new);
                            props.onchange.call(current_row.clone());
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

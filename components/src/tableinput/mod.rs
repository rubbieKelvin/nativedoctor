use std::{collections::HashMap, path::PathBuf};

use dioxus::prelude::*;
use strum::Display;

use crate::border::Border;

pub trait TableInputCell {
    fn identifier(&self) -> String;
    fn render_input(
        &self,
        value: CellValue,
        row: HashMap<String, CellValue>,
        set: impl Fn(CellValue) + 'static,
        set_partial: impl Fn(HashMap<String, CellValue>) + 'static,
    ) -> Element;
    fn internally_treat_as_empty(&self) -> bool {
        return false;
    }
}

pub fn row_is_empty<T: TableInputCell>(row: &HashMap<String, CellValue>, column: T) -> bool {
    row.iter().all(|(key, cell)| {
        // if the column for that key should be internally as empty
        (column.identifier() == *key && column.internally_treat_as_empty())
        // or the cell itself is empty...
        || matches!(cell, CellValue::Empty)
    })
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
    row_class: Option<String>,
    border: Option<Border>,
    columns: Vec<T>,
    value: Vec<HashMap<String, CellValue>>,
    onchange: Option<EventHandler<Vec<HashMap<String, CellValue>>>>,
) -> Element {
    let border = border.unwrap_or_default();
    let class = format!("{} {}", class.unwrap_or_default(), border.classes());
    let columns = use_context_provider::<Vec<T>>(|| columns);

    let remove_excess_empty_rows = |rows: &mut Vec<HashMap<String, CellValue>>,
                                    columns: &Vec<T>| {
        let empty_row_indexes = rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                if row.iter().all(|(key, cell)| {
                    // if the column for that key should be internally as empty
                    columns
                        .get(index)
                        .map(|config| {
                            config.identifier() == *key && config.internally_treat_as_empty()
                        })
                        .unwrap_or(false)
                    // or the cell itself is empty...
                    || matches!(cell, CellValue::Empty)
                })
                // if all the cells in the row passes the check above...
                {
                    return Some(index);
                }
                return None;
            })
            .collect::<Vec<usize>>();

        if empty_row_indexes.len() > 1 {
            let slice = &empty_row_indexes[0..empty_row_indexes.len() - 1];

            *rows = rows
                .iter()
                .enumerate()
                .filter_map(|(index, row)| {
                    if slice.contains(&index) {
                        return None;
                    }
                    return Some(row.clone());
                })
                .collect::<Vec<HashMap<String, CellValue>>>();
        }
    };

    let ensure_empty_row = |rows: &mut Vec<HashMap<String, CellValue>>, columns: &Vec<T>| {
        // check if there is at least one non-empty column for each row
        // meaning in every row, one column has a value in it
        let all_rows_are_satisfied = rows
            .iter()
            .all(|row| row.values().any(|value| !matches!(value, CellValue::Empty)));

        // if all rows are satisfied, let's add one new row at the end (so there's an row for the user to use)
        if all_rows_are_satisfied {
            // new empty row
            let mut new_row: HashMap<String, CellValue> = HashMap::new();

            // fill row with empty columns
            for column in columns.iter() {
                new_row.insert(column.identifier(), CellValue::Empty);
            }

            rows.push(new_row);
        }
    };

    // on mounted, ensure one empty row
    {
        let value = value.clone();
        let columns = columns.clone();

        use_hook(move || {
            let mut rows = value.clone();
            ensure_empty_row(&mut rows, &columns);

            if let Some(onchange) = onchange {
                onchange.call(rows);
            }
        })
    };

    let push_update_to_row_cell = {
        let columns = columns.clone();
        let value = value.clone();

        move |index: usize, key: String, cell_value: CellValue| {
            // we need to look at the row at index
            // ...and insert the value in the hasmap there for the specified key
            let mut rows = value.clone();

            if let Some(row) = rows.get_mut(index) {
                row.insert(key, cell_value);
            }

            // ensure there's an empty row
            ensure_empty_row(&mut rows, &columns);

            // if there are exccess empty rows remove them
            remove_excess_empty_rows(&mut rows, &columns);

            if let Some(onchange) = onchange {
                onchange.call(rows);
            }
        }
    };

    return rsx! {
        div { class,
            for (index , row) in value.iter().enumerate() {
                InputRow::<T> {
                    class: row_class.clone().unwrap_or_else(|| "flex".to_string()),
                    value: row.clone(),
                    onchange: {
                        let push_update_to_row_cell = push_update_to_row_cell.clone();
                        move |(key, cell_value)| {
                            push_update_to_row_cell(index, key, cell_value);
                        }
                    },
                }
            }
        }
    };
}

#[derive(Props, PartialEq, Clone)]
struct InputRowProps {
    class: String,
    value: HashMap<String, CellValue>,
    onchange: EventHandler<(String, CellValue)>,
}

#[component]
fn InputRow<T: TableInputCell + PartialEq + Clone + 'static>(props: InputRowProps) -> Element {
    let columns = use_context::<Vec<T>>();

    return rsx! {
        div {
            class: props.class,
            for column in columns {
                TableInputColumn::<T> {
                    column: column.clone(),
                    row: props.value.clone(),
                    value: {
                        let column_value = props.value.get(&column.identifier());
                        let column_value = column_value.map(|v| v.clone());
                        column_value.unwrap_or_default()
                    },
                    onchange: {
                        move |(key, new)| {
                            props.onchange.call((key, new));
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
    row: HashMap<String, CellValue>,
    onchange: EventHandler<(String, CellValue)>,
) -> Element {
    let identifier = column.identifier();
    return rsx! {
        {
            column
                .render_input(
                    value,
                    row.clone(),
                    move |new_value| { onchange.call((identifier.clone(), new_value)) },
                    move |new_partial_row| {
                        for (key, cell_value) in new_partial_row.iter() {
                            onchange.call((key.clone(), cell_value.clone()));
                        }
                    }
                )
        }
    };
}

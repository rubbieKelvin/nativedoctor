//! Ordered name/value pairs for headers, query and form fields.
//!
//! In YAML they're a map. A list value repeats the name, and null means "remove".

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pairs(pub Vec<(String, Option<String>)>);

impl Pairs {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, name: impl Into<String>, value: Option<String>) {
        self.0.push((name.into(), value));
    }

    pub fn remove(&mut self, name: &str, ignore_case: bool) {
        self.0.retain(|(n, _)| !names_match(n, name, ignore_case));
    }

    /// Overlays `other`: each of its names replaces all earlier values of that name.
    pub fn overlay(&mut self, other: &Pairs, ignore_case: bool) {
        let mut seen: Vec<&str> = Vec::new();
        for (name, value) in &other.0 {
            if !seen.iter().any(|s| names_match(s, name, ignore_case)) {
                self.remove(name, ignore_case);
                seen.push(name);
            }
            self.0.push((name.clone(), value.clone()));
        }
    }
}

fn names_match(a: &str, b: &str, ignore_case: bool) -> bool {
    if ignore_case {
        return a.eq_ignore_ascii_case(b);
    }
    return a == b;
}

/// Renders a YAML/JSON scalar as text. Returns `None` for maps and lists.
pub fn scalar_to_string(value: &Value) -> Option<String> {
    return match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    };
}

fn parse_value<E: serde::de::Error>(name: &str, value: &Value) -> Result<Option<String>, E> {
    if value.is_null() {
        return Ok(None);
    }
    return scalar_to_string(value)
        .map(Some)
        .ok_or_else(|| E::custom(format!("`{name}` must be a string, number, bool or null")));
}

impl<'de> Deserialize<'de> for Pairs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let map = Map::deserialize(deserializer).map_err(D::Error::custom)?;
        let mut pairs = Pairs::default();
        for (name, value) in &map {
            match value {
                Value::Array(values) => {
                    for item in values {
                        pairs.push(name, parse_value::<D::Error>(name, item)?);
                    }
                }
                other => pairs.push(name, parse_value::<D::Error>(name, other)?),
            }
        }
        return Ok(pairs);
    }
}

impl Serialize for Pairs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = Map::new();
        for (name, value) in &self.0 {
            let value = value.clone().map_or(Value::Null, Value::String);
            match map.get_mut(name) {
                None => {
                    map.insert(name.clone(), value);
                }
                Some(Value::Array(list)) => list.push(value),
                Some(first) => *first = Value::Array(vec![first.clone(), value]),
            }
        }
        return map.serialize(serializer);
    }
}

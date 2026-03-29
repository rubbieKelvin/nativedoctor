//! Convert JSON values into Rhai [`Dynamic`](rhai::Dynamic) for `json()`.

use rhai::Dynamic;
use serde_json::Value;

/// Maps `serde_json::Value` into a Rhai value (maps, arrays, scalars, unit for null).
pub(crate) fn json_to_dynamic(v: &Value) -> Dynamic {
    match v {
        Value::Null => Dynamic::UNIT,
        Value::Bool(b) => Dynamic::from(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Dynamic::from(i)
            } else if let Some(u) = n.as_u64() {
                Dynamic::from(u as i64)
            } else {
                Dynamic::from(n.as_f64().unwrap_or(0.0))
            }
        }
        Value::String(s) => Dynamic::from(s.clone()),
        Value::Array(arr) => {
            let vec: rhai::Array = arr.iter().map(json_to_dynamic).collect();
            Dynamic::from_array(vec)
        }
        Value::Object(map) => {
            let mut m = rhai::Map::new();
            for (k, v) in map {
                m.insert(k.clone().into(), json_to_dynamic(v));
            }
            Dynamic::from_map(m)
        }
    }
}

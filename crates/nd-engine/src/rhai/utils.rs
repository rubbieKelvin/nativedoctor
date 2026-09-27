//! Convert JSON values into Rhai [`Dynamic`](rhai::Dynamic) for `json()`, and the reverse for host APIs.

use base64::Engine as _;
use rhai::Dynamic;
use serde_json::{Map, Number, Value};

/// Maps `serde_json::Value` into a Rhai value (maps, arrays, scalars, unit for null).
pub fn json_to_dynamic(v: &Value) -> Dynamic {
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

/// Maps a Rhai [`Dynamic`] into [`serde_json::Value`].
///
/// Function pointers and other non-JSON values become `null` or a short diagnostic string.
/// Blobs encode as **standard base64** strings (JSON has no binary type).
pub fn dynamic_to_json(v: &Dynamic) -> Value {
    dynamic_to_json_impl(&v.flatten_clone())
}

fn dynamic_to_json_impl(v: &Dynamic) -> Value {
    if v.is_unit() {
        return Value::Null;
    }

    if v.is_bool() {
        return Value::Bool(v.as_bool().expect("is_bool implies as_bool"));
    }

    if v.is_int() {
        let i = v.as_int().expect("is_int implies as_int");
        return Value::Number(Number::from(i));
    }

    if v.is_float() {
        let f = v.as_float().expect("is_float implies as_float");
        return Number::from_f64(f)
            .map(Value::Number)
            .unwrap_or(Value::Null);
    }

    if v.is_char() {
        let c = v.as_char().expect("is_char implies as_char");
        return Value::String(c.to_string());
    }
    if v.is_string() {
        let s = v
            .as_immutable_string_ref()
            .expect("is_string implies immutable string");
        return Value::String(s.to_string());
    }
    if v.is_array() {
        let arr = v.as_array_ref().expect("is_array implies as_array_ref");
        let vec: Vec<Value> = arr.iter().map(dynamic_to_json_impl).collect();
        return Value::Array(vec);
    }
    if v.is_map() {
        let m = v.as_map_ref().expect("is_map implies as_map_ref");
        let mut out = Map::new();
        for (k, val) in m.iter() {
            out.insert(k.to_string(), dynamic_to_json_impl(val));
        }
        return Value::Object(out);
    }
    if v.is_blob() {
        let b = v.as_blob_ref().expect("is_blob implies as_blob_ref");
        return Value::String(base64::engine::general_purpose::STANDARD.encode(b.as_slice()));
    }
    if v.is_timestamp() {
        if let Some(inst) = v.clone().try_cast::<std::time::Instant>() {
            return Value::String(format!("{inst:?}"));
        }
    }
    if v.is_fnptr() {
        return Value::Null;
    }

    Value::String(format!("<{}>", v.type_name()))
}

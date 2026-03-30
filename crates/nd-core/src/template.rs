//! `${VAR}` and `${!name}` substitution: environment lookups and dynamic generators (see [`crate::env::dynamic`]).

use std::sync::OnceLock;

use regex::Regex;
use serde_json::Value;

use crate::env::dynamic;
use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

/// Placeholders: `${VAR}` (env) or `${!name}` (dynamic), `IDENT` = `[A-Za-z_][A-Za-z0-9_]*`.
fn placeholder_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    return RE.get_or_init(|| {
        Regex::new(r"\$\{(?:!([A-Za-z_][A-Za-z0-9_]*)|([A-Za-z_][A-Za-z0-9_]*))\}")
            .expect("valid regex")
    });
}

/// Replace every `${VAR}` in `input` with values from `env`, and every `${!name}` with a dynamic
/// value from [`dynamic::invoke`]. Fails if any variable is unset or the function name is unknown.
pub fn expand_string(env: &RuntimeEnv, input: &str) -> Result<String> {
    let mut out = String::with_capacity(input.len());
    let mut last = 0usize;

    for cap in placeholder_re().captures_iter(input) {
        let m = cap.get(0).expect("match");
        out.push_str(&input[last..m.start()]);

        if let Some(dyn_m) = cap.get(1) {
            out.push_str(&dynamic::invoke(dyn_m.as_str())?);
        } else if let Some(var_m) = cap.get(2) {
            let name = var_m.as_str();
            let value = env
                .get(name)
                .ok_or_else(|| Error::MissingTemplateVar(name.to_string()))?;
            out.push_str(&value);
        } else {
            unreachable!("placeholder regex must match dynamic or env capture group");
        }
        last = m.end();
    }

    out.push_str(&input[last..]);
    return Ok(out);
}

/// Recursively expand `${VAR}` / `${!name}` in JSON strings and in object keys (same rules as [`expand_string`]).
pub fn expand_json_value(env: &RuntimeEnv, value: &Value) -> Result<Value> {
    match value {
        Value::String(s) => return Ok(Value::String(expand_string(env, s)?)),
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(expand_json_value(env, item)?);
            }
            return Ok(Value::Array(out));
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                let new_k = expand_string(env, k)?;
                out.insert(new_k, expand_json_value(env, v)?);
            }
            return Ok(Value::Object(out));
        }
        _ => return Ok(value.clone()),
    }
}

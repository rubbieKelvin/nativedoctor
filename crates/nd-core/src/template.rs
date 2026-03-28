//! `${VAR}` substitution using [`crate::RuntimeEnv`] for lookups.

use std::sync::OnceLock;

use regex::Regex;
use serde_json::Value;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

/// Variable pattern: `${IDENT}` where `IDENT` matches `[A-Za-z_][A-Za-z0-9_]*`.
fn var_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*)\}").expect("valid regex"))
}

/// Replace every `${VAR}` in `input` with values from `env`. Fails if any variable is unset.
pub fn expand_string(env: &RuntimeEnv, input: &str) -> Result<String> {
    let mut out = String::with_capacity(input.len());
    let mut last = 0usize;
    for cap in var_re().captures_iter(input) {
        let m = cap.get(0).expect("match");
        out.push_str(&input[last..m.start()]);
        let name = cap.get(1).expect("group 1").as_str();
        let value = env
            .get(name)
            .ok_or_else(|| Error::MissingTemplateVar(name.to_string()))?;
        out.push_str(&value);
        last = m.end();
    }
    out.push_str(&input[last..]);
    Ok(out)
}

/// Recursively expand `${VAR}` in JSON strings and in object keys (same rules as [`expand_string`]).
pub fn expand_json_value(env: &RuntimeEnv, value: &Value) -> Result<Value> {
    match value {
        Value::String(s) => Ok(Value::String(expand_string(env, s)?)),
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(expand_json_value(env, item)?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                let new_k = expand_string(env, k)?;
                out.insert(new_k, expand_json_value(env, v)?);
            }
            Ok(Value::Object(out))
        }
        _ => Ok(value.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn expand_simple() {
        let env = RuntimeEnv::from_process_env();
        env.set_runtime("FOO", "bar");
        assert_eq!(expand_string(&env, "${FOO}").unwrap(), "bar");
    }

    #[test]
    fn expand_missing() {
        let env = RuntimeEnv::from_process_env();
        assert!(expand_string(&env, "${__UNLIKELY_VAR_XYZ__}").is_err());
    }

    #[test]
    fn expand_json_nested() {
        let env = RuntimeEnv::from_process_env();
        env.set_runtime("X", "1");
        let v = json!({ "a": "${X}", "b": [ "${X}" ] });
        let e = expand_json_value(&env, &v).unwrap();
        assert_eq!(e, json!({ "a": "1", "b": ["1"] }));
    }
}

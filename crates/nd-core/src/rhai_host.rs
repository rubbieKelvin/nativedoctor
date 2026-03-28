//! Rhai **post-response** scripts: no filesystem or network inside the engine.
//!
//! # Built-in functions
//!
//! | Function | Returns | Notes |
//! |----------|---------|--------|
//! | `response_status()` | `i64` | HTTP status code |
//! | `response_header(name)` | value or `()` | Header name is case-sensitive as stored |
//! | `response_body_string()` | `string` | UTF-8 lossy over raw bytes |
//! | `response_body_json()` | value or `()` | Parsed JSON as Rhai value; `()` if body is not valid JSON |
//! | `env(key)` | value or `()` | Uses [`crate::RuntimeEnv::get`] |
//! | `set_runtime(key, value)` | — | Updates runtime map via [`crate::RuntimeEnv::set_runtime`]; `value` is stringified |

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use rhai::{Dynamic, Engine};
use serde_json::Value;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

/// Snapshot of the HTTP response passed into Rhai (immutable inside the script run).
#[derive(Clone)]
struct ResponseCtx {
    status: i64,
    headers: HashMap<String, String>,
    body_str: String,
    json_value: Option<Value>,
}

/// Convert `serde_json::Value` to Rhai `Dynamic` (maps, arrays, scalars).
fn json_to_dynamic(v: &Value) -> Dynamic {
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

/// Compile and run the Rhai script at `script_path` after a response is available.
///
/// `headers` should be the response header list (name/value). `body` is the raw response body;
/// a best-effort UTF-8 string is also exposed separately from parsed JSON.
pub fn run_post_script(
    script_path: &Path,
    env: &RuntimeEnv,
    status: u16,
    headers: &[(String, String)],
    body: &[u8],
) -> Result<()> {
    let source = std::fs::read_to_string(script_path).map_err(|_| {
        Error::PostScriptNotFound(script_path.to_path_buf())
    })?;

    let header_map: HashMap<String, String> = headers.iter().cloned().collect();
    let body_str = String::from_utf8_lossy(body).to_string();
    let json_value = serde_json::from_slice(body).ok();

    let ctx = Arc::new(ResponseCtx {
        status: status as i64,
        headers: header_map,
        body_str,
        json_value,
    });

    let mut engine = Engine::new();

    let c = ctx.clone();
    engine.register_fn("response_status", move || c.status);

    let c = ctx.clone();
    engine.register_fn("response_header", move |name: &str| {
        c.headers
            .get(name)
            .map(|s| Dynamic::from(s.clone()))
            .unwrap_or(Dynamic::UNIT)
    });

    let c = ctx.clone();
    engine.register_fn("response_body_string", move || c.body_str.clone());

    let c = ctx.clone();
    engine.register_fn("response_body_json", move || match &c.json_value {
        Some(v) => json_to_dynamic(v),
        None => Dynamic::UNIT,
    });

    let e_env = env.clone();
    engine.register_fn("env", move |key: &str| {
        e_env
            .get(key)
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    });

    let e_set = env.clone();
    engine.register_fn("set_runtime", move |key: &str, value: Dynamic| {
        e_set.set_runtime(key.to_string(), value.to_string());
    });

    let mut scope = rhai::Scope::new();
    engine
        .run_with_scope(&mut scope, &source)
        .map_err(|e| Error::Rhai(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn post_script_sets_runtime_from_json() {
        let dir = tempdir().unwrap();
        let script = dir.path().join("t.rhai");
        std::fs::write(
            &script,
            r#"let j = response_body_json(); set_runtime("K", j.id);"#,
        )
        .unwrap();
        let env = RuntimeEnv::from_process_env();
        run_post_script(
            &script,
            &env,
            200,
            &[("Content-Type".into(), "application/json".into())],
            br#"{"id":"xyz"}"#,
        )
        .unwrap();
        assert_eq!(env.get("K").as_deref(), Some("xyz"));
    }
}

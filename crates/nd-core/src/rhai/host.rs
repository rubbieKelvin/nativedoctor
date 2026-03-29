//! Rhai **post-response** scripts: no filesystem or network inside the engine.
//!
//! # Built-in functions
//!
//! | Function | Returns | Notes |
//! |----------|---------|--------|
//! | `status()` | `i64` | HTTP status code |
//! | `headers(name)` | value or `()` | Header name is case-sensitive as stored |
//! | `body()` | `string` | UTF-8 lossy over raw bytes |
//! | `json()` | value or `()` | Parsed JSON as Rhai value; `()` if body is not valid JSON |
//! | `env(key)` | value or `()` | Uses [`crate::RuntimeEnv::get`] |
//! | `set(key, value)` | — | Updates runtime map via [`crate::RuntimeEnv::set_runtime`]; `value` is stringified |
//! | `log(level, message)` | — | When a [`crate::rhai::Logger`] is passed to [`run_post_script`], appends a [`crate::rhai::Log`] (unknown `level` → `info`) |

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use super::Logger;

use rhai::{Dynamic, Engine};
use serde_json::Value;
use tracing::debug;

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
    logger: Option<Arc<Logger>>,
) -> Result<()> {
    let source = std::fs::read_to_string(script_path)
        .map_err(|_| Error::PostScriptNotFound(script_path.to_path_buf()))?;

    debug!(
        path = %script_path.display(),
        status,
        body_len = body.len(),
        header_count = headers.len(),
        "Rhai post_script evaluating"
    );

    let header_map: HashMap<String, String> = headers.iter().cloned().collect();
    let body_str = String::from_utf8_lossy(body).to_string();
    let json_value = serde_json::from_slice(body).ok();

    let ctx = Arc::new(ResponseCtx {
        status: status as i64,
        headers: header_map,
        body_str,
        json_value,
    });

    let mut scope = rhai::Scope::new();
    let engine = create_engine(ctx, env, script_path, logger);

    engine
        .run_with_scope(&mut scope, &source)
        .map_err(|e| Error::Rhai(e.to_string()))?;

    debug!(path = %script_path.display(), "Rhai post_script finished");
    Ok(())
}

fn create_engine(
    ctx: Arc<ResponseCtx>,
    env: &RuntimeEnv,
    script_path: &Path,
    logger: Option<Arc<Logger>>,
) -> Engine {
    let mut engine = Engine::new();

    let c = ctx.clone();
    engine.register_fn("status", move || c.status);

    let c = ctx.clone();
    engine.register_fn("headers", move |name: &str| {
        c.headers
            .get(name)
            .map(|s| Dynamic::from(s.clone()))
            .unwrap_or(Dynamic::UNIT)
    });

    let c = ctx.clone();
    engine.register_fn("body", move || c.body_str.clone());

    let c = ctx.clone();
    engine.register_fn("json", move || match &c.json_value {
        Some(v) => json_to_dynamic(v),
        None => Dynamic::UNIT,
    });

    let e_env = env.clone();
    engine.register_fn("env", move |key: &str| {
        e_env.get(key).map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    });

    let e_set = env.clone();
    engine.register_fn("set", move |key: &str, value: Dynamic| {
        e_set.set_runtime(key.to_string(), value.to_string());
    });

    if let Some(sink) = logger {
        let sink = sink.clone();
        let script_label = script_path.display().to_string();
        engine.register_fn("log", move |level: &str, message: &str| {
            sink.log_parsed_level(level, message, script_label.clone(), "post_script");
        });
    }

    engine
}

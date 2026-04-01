use std::path::Path;
use std::sync::Arc;

use rhai::{Dynamic, Engine, EvalAltResult, Position};

use super::logger::{emit_script_log_to_tracing, LogLevel, Logger};

use crate::env::RuntimeEnv;

/// Registers `env` and `set` for template/runtime map access.
fn register_env_fns(engine: &mut Engine, env: &RuntimeEnv) {
    let e_env = env.clone();
    engine.register_fn("env", move |key: &str| {
        e_env.get(key).map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    });

    let e_set = env.clone();
    engine.register_fn("set", move |key: &str, value: Dynamic| {
        e_set.set(key.to_string(), value.to_string());
    });
}

/// Registers `assert(condition, message)` — fails script evaluation when `condition` is false.
fn register_assert(engine: &mut Engine) {
    engine.register_fn("assert", |condition: bool, message: &str| {
        if condition {
            Ok(())
        } else {
            Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("assertion failed: {message}").into(),
                Position::NONE,
            )))
        }
    });
}

/// Registers `log(level, message)` — always traces; optionally appends to `logger`.
fn register_log(engine: &mut Engine, logger: Option<Arc<Logger>>, script_label: String) {
    let sink = logger;
    engine.register_fn("log", move |level: &str, message: &str| {
        let lvl = LogLevel::parse_or_info(level);
        emit_script_log_to_tracing(lvl, &script_label, message);
        if let Some(ref s) = sink {
            s.log_parsed_level(level, message, script_label.clone(), "");
        }
    });
}

/// Registers `persist(key, value)` when `persist_file` is set — updates env and `runtime.nativedoctor.json`.
fn register_persist(engine: &mut Engine, env: &RuntimeEnv) {
    let e = env.clone();
    engine.register_fn("persist", move |key: &str, value: Dynamic| {
        let s = value.to_string();
        e.persist(key, &s).map_err(|err| {
            Box::new(EvalAltResult::ErrorRuntime(
                format!("persist failed: {err}").into(),
                Position::NONE,
            ))
        })
    });
}

/// Creates the post-script engine: no raw network inside Rhai; optional `persist` writes the persist file.
pub(crate) fn create_engine(
    env: &RuntimeEnv,
    script_path: &Path,
    logger: Option<Arc<Logger>>,
) -> Engine {
    let mut engine = Engine::new();
    let script_label = script_path.display().to_string();

    register_env_fns(&mut engine, env);
    register_assert(&mut engine);
    register_log(&mut engine, logger, script_label);
    register_persist(&mut engine, env);
    return engine;
}

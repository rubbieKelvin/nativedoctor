use std::path::Path;
use std::sync::Arc;

use rhai::{Dynamic, Engine, EvalAltResult, FuncRegistration, Position};

use super::logger::{emit_script_log_to_tracing, LogLevel, Logger};
use super::resolver::{NativeImportResolver, RhaiScriptRunOptions};

use crate::env::RuntimeEnv;

/// Registers `env` and `set` for template/runtime map access.
fn register_env_fns(engine: &mut Engine, env: &RuntimeEnv) {
    let e_env = env.clone();
    FuncRegistration::new("env")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Look up a runtime variable (for `${VAR}` expansion and `env()` in scripts).",
            "/// Returns `()` if the key is not set.",
        ])
        .register_into_engine(engine, move |key: &str| {
            e_env.get(key).map(Dynamic::from).unwrap_or(Dynamic::UNIT)
        });

    let e_set = env.clone();
    FuncRegistration::new("set")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Set a runtime variable (value is stringified). Visible to later templates and `env()`.",
        ])
        .register_into_engine(engine, move |key: &str, value: Dynamic| {
            e_set.set(key.to_string(), value.to_string());
        });
}

/// Registers `assert(condition, message)` — fails script evaluation when `condition` is false.
fn register_assert(engine: &mut Engine) {
    FuncRegistration::new("assert")
        .in_global_namespace()
        .with_comments([
            "/// Fail evaluation with a runtime error if `condition` is false.",
        ])
        .register_into_engine(engine, |condition: bool, message: &str| {
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
    FuncRegistration::new("log")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Log a message at the given level (e.g. `\"info\"`, `\"debug\"`).",
        ])
        .register_into_engine(engine, move |level: &str, message: &str| {
            let lvl = LogLevel::parse_or_info(level);
            emit_script_log_to_tracing(lvl, &script_label, message);
            if let Some(ref s) = sink {
                s.log_parsed_level(level, message, script_label.clone(), "");
            }
        });
}

/// Registers `persist(key, value)` when `persist_file` is set — updates env and the configured persistence file (JSON or YAML).
fn register_persist(engine: &mut Engine, env: &RuntimeEnv) {
    let e = env.clone();

    FuncRegistration::new("persist")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Persist a key–value pair to the configured persistence file (if any).",
        ])
        .register_into_engine(engine, move |key: &str, value: Dynamic| {
            let s = value.to_string();
            e.persist(key, &s).map_err(|err| {
                Box::new(EvalAltResult::ErrorRuntime(
                    format!("persist failed: {err}").into(),
                    Position::NONE,
                ))
            })
        });
}

/// Creates the script engine: builtins, `import` resolution (`.rhai` and request files), optional `persist`.
pub(crate) fn create_engine(
    env: &RuntimeEnv,
    script_path: &Path,
    logger: Option<Arc<Logger>>,
    script_options: RhaiScriptRunOptions,
) -> Engine {
    let mut engine = Engine::new();
    let script_label = script_path.display().to_string();

    // register functions
    register_env_fns(&mut engine, env);
    register_assert(&mut engine);
    register_log(&mut engine, logger, script_label);
    register_persist(&mut engine, env);

    let env_arc = Arc::new(env.clone());
    let resolver = NativeImportResolver::new(script_path, env_arc, script_options);
    engine.set_module_resolver(resolver);

    return engine;
}

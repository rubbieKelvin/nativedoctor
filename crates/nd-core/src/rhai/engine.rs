use std::path::Path;
use std::sync::{Arc, Mutex};

use rhai::{Dynamic, Engine, EvalAltResult, FuncRegistration, Position};

use super::logger::{emit_script_log_to_tracing, LogLevel};
use super::resolver::{NativeImportResolver, RhaiScriptRunOptions};

use crate::rhai::utils::dynamic_to_json;
use crate::stream::events::Event;
use crate::stream::{MutexSession, Session};

/// Registers `env` and `set` for template/runtime map access.
fn register_env_fns(engine: &mut Engine, session: Arc<Mutex<Session>>) {
    let e_env = session.runtime();

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

    let e_set = session.runtime();
    let session_for_set = session.clone();

    FuncRegistration::new("set")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Set a runtime variable (value is stringified). Visible to later templates and `env()`.",
        ])
        .register_into_engine(engine, move |key: &str, value: Dynamic| {
            e_set.set(key.to_string(), value.to_string());

            let json = dynamic_to_json(&value);
            let key_owned = key.to_string();

            session_for_set
                .emit(|elapsed| Event::RuntimeVariablePushed {
                    elapsed,
                    key: key_owned,
                    value: json,
                    persisted: false
                });
        });
}

/// Registers `assert(condition, message)` — fails script evaluation when `condition` is false.
fn register_assert(engine: &mut Engine, session: Arc<Mutex<Session>>) {
    FuncRegistration::new("assert")
        .in_global_namespace()
        .with_comments(["/// Fail evaluation with a runtime error if `condition` is false."])
        .register_into_engine(engine, move |condition: bool, message: &str| {
            let msg = message.to_string();

            session.emit(|elapsed| Event::AssertCalled {
                passed: condition,
                elapsed,
                message: msg,
            });

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

/// Registers `log(level, message)` — traces to the tracing subsystem and [`Session::log`] (timeline / [`crate::stream::events::Event::Log`]).
fn register_log(engine: &mut Engine, session: Arc<Mutex<Session>>, script_label: String) {
    FuncRegistration::new("log")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments(["/// Log a message at the given level (e.g. `\"info\"`, `\"debug\"`)."])
        .register_into_engine(engine, move |level: &str, message: &str| {
            let lvl = LogLevel::parse_or_info(level);

            emit_script_log_to_tracing(lvl, &script_label, message);
            let msg = message.to_string();
            let label = script_label.clone();

            session.emit(|elapsed| Event::Log {
                level: lvl,
                message: msg,
                script: label,
                elapsed,
            });
        });
}

/// Registers `persist(key, value)` when `persist_file` is set — updates env and the configured persistence file (JSON or YAML).
fn register_persist(engine: &mut Engine, session: Arc<Mutex<Session>>) {
    let e = session.runtime();
    let session_e = session.clone();

    FuncRegistration::new("persist")
        .in_global_namespace()
        .with_volatility(true)
        .with_comments([
            "/// Persist a key–value pair to the configured persistence file (if any).",
        ])
        .register_into_engine(engine, move |key: &str, value: Dynamic| {
            let s = value.to_string();
            let key_owned = key.to_string();
            let json = dynamic_to_json(&value);

            session_e.emit(|elapsed| Event::RuntimeVariablePushed {
                elapsed,
                key: key_owned,
                value: json,
                persisted: false,
            });

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
    session: Arc<Mutex<Session>>,
    script_path: &Path,
    script_options: RhaiScriptRunOptions,
) -> Engine {
    let mut engine = Engine::new();
    let script_label = script_path.display().to_string();

    register_env_fns(&mut engine, session.clone());
    register_assert(&mut engine, session.clone());
    register_log(&mut engine, session.clone(), script_label);
    register_persist(&mut engine, session.clone());

    let env_arc = Arc::new(
        session
            .lock()
            .expect("session mutex poisoned")
            .runtime
            .clone(),
    );
    let resolver = NativeImportResolver::new(script_path, env_arc, script_options);
    engine.set_module_resolver(resolver);

    return engine;
}

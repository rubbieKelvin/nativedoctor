//! Build a locked-down Rhai [`Engine`](rhai::Engine) with response, env, and `log` hooks.

use std::path::Path;
use std::sync::Arc;

use nd_constants::RHAI_LOG_INITIATOR;
use rhai::{Dynamic, Engine};

use super::context::ResponseCtx;
use super::json_dynamic::json_to_dynamic;
use super::logger::{emit_script_log_to_tracing, LogLevel, Logger};

use crate::env::RuntimeEnv;

/// Registers `status`, `headers`, `body`, and `json` against `ctx`.
fn register_response_fns(engine: &mut Engine, ctx: Arc<ResponseCtx>) {
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
}

/// Registers `env` and `set` for template/runtime map access.
fn register_env_fns(engine: &mut Engine, env: &RuntimeEnv) {
    let e_env = env.clone();
    engine.register_fn("env", move |key: &str| {
        e_env.get(key).map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    });

    let e_set = env.clone();
    engine.register_fn("set", move |key: &str, value: Dynamic| {
        e_set.set_runtime(key.to_string(), value.to_string());
    });
}

/// Registers `log(level, message)` — always traces; optionally appends to `logger`.
fn register_log(engine: &mut Engine, logger: Option<Arc<Logger>>, script_label: String) {
    let sink = logger;
    engine.register_fn("log", move |level: &str, message: &str| {
        let lvl = LogLevel::parse_or_info(level);
        emit_script_log_to_tracing(lvl, &script_label, message);
        if let Some(ref s) = sink {
            s.log_parsed_level(level, message, script_label.clone(), RHAI_LOG_INITIATOR);
        }
    });
}

/// Creates the post-script engine: no filesystem or network inside Rhai.
pub(crate) fn create_engine(
    ctx: Arc<ResponseCtx>,
    env: &RuntimeEnv,
    script_path: &Path,
    logger: Option<Arc<Logger>>,
) -> Engine {
    let mut engine = Engine::new();
    register_response_fns(&mut engine, ctx);
    register_env_fns(&mut engine, env);
    let script_label = script_path.display().to_string();
    register_log(&mut engine, logger, script_label);
    engine
}

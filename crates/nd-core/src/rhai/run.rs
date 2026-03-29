//! Orchestrate reading a post-script, building [`ResponseCtx`](super::context::ResponseCtx), and running Rhai.

use std::path::Path;
use std::sync::Arc;

use tracing::debug;

use super::context::ResponseCtx;
use super::engine::create_engine;
use super::logger::Logger;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

/// Rhai **post-response** scripts: no filesystem or network inside the engine.
///
/// # Built-in functions
///
/// | Function | Returns | Notes |
/// |----------|---------|--------|
/// | `status()` | `i64` | HTTP status code |
/// | `headers(name)` | value or `()` | Header name is case-sensitive as stored |
/// | `body()` | `string` | UTF-8 lossy over raw bytes |
/// | `json()` | value or `()` | Parsed JSON as Rhai value; `()` if body is not valid JSON |
/// | `env(key)` | value or `()` | Uses [`crate::RuntimeEnv::get`] |
/// | `set(key, value)` | — | Updates runtime map via [`crate::RuntimeEnv::set_runtime`]; `value` is stringified |
/// | `log(level, message)` | — | Always emits [`tracing`]; if a [`Logger`] is passed, also records a [`super::logger::Log`]. Unknown `level` → `info`. |
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

    let ctx = Arc::new(ResponseCtx::from_parts(status, headers, body));
    let mut scope = rhai::Scope::new();
    let engine = create_engine(ctx, env, script_path, logger);

    engine
        .run_with_scope(&mut scope, &source)
        .map_err(|e| Error::Rhai(e.to_string()))?;

    debug!(path = %script_path.display(), "Rhai post_script finished");
    Ok(())
}

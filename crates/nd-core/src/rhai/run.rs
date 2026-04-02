//! Orchestrate reading a post-script, building [`ResponseCtx`](super::context::ResponseCtx), and running Rhai.

use std::path::Path;
use std::sync::Arc;

use tracing::debug;

use super::engine::create_engine;
use super::logger::Logger;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

pub fn run_rhai_script(path: &Path, env: &RuntimeEnv, logger: Option<Arc<Logger>>) -> Result<()> {
    let source =
        std::fs::read_to_string(path).map_err(|_| Error::PostScriptNotFound(path.to_path_buf()))?;

    debug!(
        path = %path.display(),
        "Rhai post_script evaluating"
    );

    let mut scope = rhai::Scope::new();
    let engine = create_engine(env, path, logger);

    engine
        .run_with_scope(&mut scope, &source)
        .map_err(|e| Error::Rhai(e.to_string()))?;

    debug!(path = %path.display(), "Rhai post_script finished");
    return Ok(());
}

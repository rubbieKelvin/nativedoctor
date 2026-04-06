//! Orchestrate compiling a Rhai script from disk (so `import` sees a real file source) and running it.

use std::path::Path;
use std::sync::{Arc, Mutex};

use tracing::debug;

use super::engine::create_engine;
use super::resolver::RhaiScriptRunOptions;

use crate::error::{Error, Result};
use crate::stream::Session;

pub fn run_rhai_script(
    path: &Path,
    session: Arc<Mutex<Session>>,
    script_options: RhaiScriptRunOptions,
) -> Result<()> {
    if !path.is_file() {
        return Err(Error::PostScriptNotFound(path.to_path_buf()));
    }

    // Absolute path so module resolution and Rhai's `global.source()` agree on the script directory
    // regardless of the process working directory.
    let script_path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    debug!(
        path = %script_path.display(),
        "Rhai script evaluating"
    );

    let mut scope = rhai::Scope::new();
    let engine = create_engine(session.clone(), &script_path, script_options);

    let ast = engine
        .compile_file_with_scope(&scope, script_path.clone())
        .map_err(|e| Error::Rhai(e.to_string()))?;

    engine
        .run_ast_with_scope(&mut scope, &ast)
        .map_err(|e| Error::Rhai(e.to_string()))?;

    debug!(path = %script_path.display(), "Rhai script finished");
    return Ok(());
}

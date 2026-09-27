//! Emit Rhai [definition files](https://rhai.rs/book/engine/metadata/index.html) (`.d.rhai`) for IDEs / language servers.

use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

use nd_core::env::RuntimeEnv;

use super::engine::create_engine;
use super::resolver::RhaiScriptRunOptions;
use crate::stream::Session;

/// Writes Rhai definition files into `out_dir` (creates the directory).
///
/// This uses [`Engine::definitions`](rhai::Engine::definitions) from Rhai (builtins + this crate's
/// globals: `env`, `set`, `assert`, `log`, `persist`).
pub fn write_rhai_definition_files(out_dir: &Path) -> io::Result<()> {
    let stub = Path::new(".nativedoctor/stub.rhai");
    let engine = create_engine(
        Arc::new(Mutex::new(
            Session::new(|| Ok(RuntimeEnv::new()), None)
                .map_err(io::Error::other)?,
        )),
        stub,
        RhaiScriptRunOptions::default(),
    );
    engine.definitions().write_to_dir(out_dir)?;
    return Ok(());
}

/// Merged single-file definitions (Rhai builtins + nativedoctor globals).
pub fn rhai_definitions_single_file() -> String {
    let stub = Path::new(".nativedoctor/stub.rhai");
    let engine = create_engine(
        Arc::new(Mutex::new(
            Session::new(|| Ok(RuntimeEnv::new()), None).expect("runtime session"),
        )),
        stub,
        RhaiScriptRunOptions::default(),
    );

    engine.definitions().single_file()
}

/// Writes [`rhai_definitions_single_file`] to `path` (overwrites).
pub fn write_rhai_definitions_file(path: &Path) -> io::Result<()> {
    return std::fs::write(path, rhai_definitions_single_file());
}

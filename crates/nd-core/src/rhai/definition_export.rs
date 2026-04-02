//! Emit Rhai [definition files](https://rhai.rs/book/engine/metadata/index.html) (`.d.rhai`) for IDEs / language servers.

use std::fs;
use std::io;
use std::path::Path;

use crate::env::RuntimeEnv;

use super::engine::create_engine;
use super::RhaiScriptRunOptions;

static REQUEST_IMPORT_SUPPLEMENT: &str = include_str!("../../definitions/nativedoctor-request.d.rhai");

/// Writes Rhai definition files into `out_dir` (creates the directory).
///
/// This uses [`Engine::definitions`](rhai::Engine::definitions) from Rhai (builtins + this crate’s globals),
/// then adds `nativedoctor-request.d.rhai` describing
/// `import` of request files (`invoke`).
///
/// Typical layout for editor support:
///
/// ```text
/// .rhai/definitions/
///   __builtin__.d.rhai
///   __builtin-operators__.d.rhai
///   __static__.d.rhai
///   nativedoctor-request.d.rhai
/// ```
pub fn write_rhai_definition_files(out_dir: &Path) -> io::Result<()> {
    let env = RuntimeEnv::new();
    let stub = Path::new(".nativedoctor/stub.rhai");
    let engine = create_engine(&env, stub, None, RhaiScriptRunOptions::default());
    engine.definitions().write_to_dir(out_dir)?;
    fs::write(
        out_dir.join("nativedoctor-request.d.rhai"),
        REQUEST_IMPORT_SUPPLEMENT,
    )?;
    Ok(())
}

/// Merged single-file definitions (Rhai builtins + nativedoctor globals), plus the request-import supplement.
///
/// Equivalent to [`Definitions::single_file`] with extra content appended.
pub fn rhai_definitions_single_file() -> String {
    let env = RuntimeEnv::new();
    let stub = Path::new(".nativedoctor/stub.rhai");
    let engine = create_engine(&env, stub, None, RhaiScriptRunOptions::default());
    let mut s = engine.definitions().single_file();
    s.push_str("\n\n");
    s.push_str(REQUEST_IMPORT_SUPPLEMENT);
    s
}

/// Writes [`rhai_definitions_single_file`] to `path` (overwrites).
pub fn write_rhai_definitions_file(path: &Path) -> io::Result<()> {
    fs::write(path, rhai_definitions_single_file())
}

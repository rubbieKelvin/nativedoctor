//! `nativedoctor rhai-definitions` — emit Rhai `.d.rhai` stubs for editor / LSP support.

use std::path::PathBuf;

use nd_core::rhai::definition_export::{write_rhai_definition_files, write_rhai_definitions_file};

#[derive(Debug, Clone)]
pub struct RhaiDefinitionsOptions {
    /// Directory to write multiple `.d.rhai` files (Rhai layout: `__builtin__.d.rhai`, …).
    pub out_dir: Option<PathBuf>,
    /// Single merged file path (alternative to `out_dir`).
    pub out_file: Option<PathBuf>,
}

pub fn run_rhai_definitions(opts: RhaiDefinitionsOptions) -> Result<(), String> {
    match (&opts.out_dir, &opts.out_file) {
        (Some(dir), None) => write_rhai_definition_files(dir).map_err(|e| e.to_string()),
        (None, Some(file)) => write_rhai_definitions_file(file).map_err(|e| e.to_string()),
        (Some(_), Some(_)) => Err(String::from("use either --out-dir or --out-file, not both")),
        (None, None) => Err(String::from(
            "specify --out-dir or --out-file (see nativedoctor rhai-definitions --help)",
        )),
    }
}

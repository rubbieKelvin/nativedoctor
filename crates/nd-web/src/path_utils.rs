//! Safe resolution of request/sequence file paths under the configured workspace root.

use std::path::{Path, PathBuf};

use dioxus::fullstack::ServerFnError;

/// Resolve `name` as a single path segment under `root` (basename only; no `..`).
pub fn resolve_basename_under_root(root: &Path, name: &str) -> Result<PathBuf, ServerFnError> {
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(ServerFnError::new("invalid file name"));
    }
    let p = root.join(name);
    if !p.is_file() {
        return Err(ServerFnError::new("file not found"));
    }
    let root_canon = std::fs::canonicalize(root).map_err(|e| ServerFnError::new(e.to_string()))?;
    let file_canon = std::fs::canonicalize(&p).map_err(|e| ServerFnError::new(e.to_string()))?;
    if !file_canon.starts_with(&root_canon) {
        return Err(ServerFnError::new("path escapes workspace"));
    }
    Ok(file_canon)
}

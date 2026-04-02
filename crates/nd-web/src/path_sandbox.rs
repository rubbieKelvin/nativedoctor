//! Resolve and verify paths stay under configured workspace roots.

use std::path::{Path, PathBuf};

/// Canonicalize each directory root; skip missing dirs with an error string for caller to surface.
pub fn canonicalize_roots(roots: &[PathBuf]) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::with_capacity(roots.len());
    for r in roots {
        let meta = std::fs::metadata(r).map_err(|e| format!("{}: {e}", r.display()))?;
        if !meta.is_dir() {
            return Err(format!("not a directory: {}", r.display()));
        }
        let c = std::fs::canonicalize(r).map_err(|e| format!("{}: {e}", r.display()))?;
        out.push(c);
    }
    Ok(out)
}

/// Returns canonical path if `candidate` is a file under one of `roots`.
pub fn resolve_allowed_file(candidate: &Path, roots: &[PathBuf]) -> Result<PathBuf, String> {
    let meta = std::fs::metadata(candidate).map_err(|e| e.to_string())?;
    if !meta.is_file() {
        return Err("not a file".into());
    }
    let canon = std::fs::canonicalize(candidate).map_err(|e| e.to_string())?;
    if is_under_roots(&canon, roots) {
        Ok(canon)
    } else {
        Err("path outside workspace roots".into())
    }
}

/// Returns true if `canon` is equal to or nested under one of the canonical roots.
pub fn is_under_roots(canon: &Path, roots: &[PathBuf]) -> bool {
    roots.iter().any(|root| canon.starts_with(root))
}

//! Path sandbox for workspace roots: every file read or script executed must resolve under a canonical root.
//!
//! This mirrors the threat model of a local dev tool: users pass explicit directories; the server must not
//! follow arbitrary paths from the client (e.g. `/etc/passwd` or paths outside the workspace).

use std::path::{Path, PathBuf};

/// Canonicalize each configured root directory.
///
/// Returns an error string if a path is missing, not a directory, or cannot be canonicalized.
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

/// Resolve `candidate` to a canonical path and ensure it is a **file** under one of `roots`.
///
/// Used for `/api/file` and execute endpoints so symlink targets outside the workspace are rejected
/// after canonicalization.
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

/// `true` if `canon` is exactly a root or nested under one (component-wise prefix match).
pub fn is_under_roots(canon: &Path, roots: &[PathBuf]) -> bool {
    roots.iter().any(|root| canon.starts_with(root))
}

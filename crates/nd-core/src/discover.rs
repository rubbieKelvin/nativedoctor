//! Non-recursive discovery of request definition files in a single directory.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Result;

/// Whether a workspace file is a single-request document, a sequence, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceFileKind {
    Request,
    Script,
}

fn is_request_file(path: &Path) -> bool {
    return path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| matches!(e.to_lowercase().as_str(), "json" | "yaml" | "yml"))
        .unwrap_or(false);
}

/// List `*.json`, `*.yaml`, and `*.yml` files in `dir` only (not subdirectories).
///
/// Missing `dir` yields an empty list. Paths are deduplicated and sorted.
pub fn list_request_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let read = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(paths),
        Err(e) => return Err(crate::error::Error::Io(e)),
    };

    for entry in read {
        let entry = entry.map_err(crate::error::Error::Io)?;
        let p = entry.path();
        if p.is_file() && is_request_file(&p) {
            paths.push(p);
        }
    }

    let unique: BTreeSet<PathBuf> = paths.into_iter().collect();
    let mut sorted: Vec<PathBuf> = unique.into_iter().collect();
    sorted.sort();

    debug!(
        dir = %dir.display(),
        count = sorted.len(),
        "list_request_paths"
    );
    return Ok(sorted);
}

/// Like [`list_request_paths`], but each path is classified. Files that fail to read or parse are skipped with a warning.
pub fn list_workspace_catalog(dir: &Path) -> Result<Vec<(PathBuf, WorkspaceFileKind)>> {
    let paths = list_request_paths(dir)?;
    let /*mut*/ out = Vec::with_capacity(paths.len());
    for _p in paths {
        todo!("Finish writing this function");
    }
    Ok(out)
}

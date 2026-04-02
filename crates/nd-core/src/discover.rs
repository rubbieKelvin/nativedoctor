//! Non-recursive discovery of request definition files and Rhai scripts in a single directory.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Result;
use crate::model::request::RequestFile;

fn is_request_file(path: &Path) -> bool {
    return path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| matches!(e.to_lowercase().as_str(), "json" | "yaml" | "yml"))
        .unwrap_or(false);
}

fn is_rhai_file(path: &Path) -> bool {
    return path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("rhai"))
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

/// List `*.rhai` files in `dir` only (not subdirectories).
///
/// Missing `dir` yields an empty list. Paths are deduplicated and sorted.
pub fn list_rhai_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let read = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(paths),
        Err(e) => return Err(crate::error::Error::Io(e)),
    };

    for entry in read {
        let entry = entry.map_err(crate::error::Error::Io)?;
        let p = entry.path();
        if p.is_file() && is_rhai_file(&p) {
            paths.push(p);
        }
    }

    let unique: BTreeSet<PathBuf> = paths.into_iter().collect();
    let mut sorted: Vec<PathBuf> = unique.into_iter().collect();
    sorted.sort();

    debug!(
        dir = %dir.display(),
        count = sorted.len(),
        "list_rhai_paths"
    );
    return Ok(sorted);
}

/// A request file path that failed [`RequestFile::from_file`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedRequestFile {
    pub path: PathBuf,
    pub message: String,
}

/// Partition paths into those that load as [`RequestFile`] and those that do not.
///
/// Order of `valid` matches the order of `paths` (stable); skipped entries preserve order of first failure per path.
pub fn partition_valid_request_paths(paths: &[PathBuf]) -> (Vec<PathBuf>, Vec<SkippedRequestFile>) {
    let mut valid = Vec::new();
    let mut skipped = Vec::new();
    for p in paths {
        match RequestFile::from_file(p) {
            Ok(_) => valid.push(p.clone()),
            Err(e) => skipped.push(SkippedRequestFile {
                path: p.clone(),
                message: e.to_string(),
            }),
        }
    }
    return (valid, skipped);
}

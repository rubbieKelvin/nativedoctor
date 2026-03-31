//! Non-recursive discovery of request definition files in a single directory.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing::warn;

use crate::error::Result;

/// Whether a workspace file is a single-request document, a sequence, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceFileKind {
    Request,
    Sequence,
    Unknown,
}

fn is_request_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| matches!(e.to_lowercase().as_str(), "json" | "yaml" | "yml"))
        .unwrap_or(false)
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
    Ok(sorted)
}

fn parse_as_json_value(path: &Path, text: &str) -> crate::error::Result<serde_json::Value> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "json" => serde_json::from_str(text).map_err(|source| crate::error::Error::ParseJson {
            path: path.to_path_buf(),
            source,
        }),
        "yaml" | "yml" => {
            serde_yaml::from_str(text).map_err(|source| crate::error::Error::ParseYaml {
                path: path.to_path_buf(),
                source,
            })
        }
        _ => Err(crate::error::Error::UnsupportedFormat(path.to_path_buf())),
    }
}

fn classify_json_document(v: &serde_json::Value) -> WorkspaceFileKind {
    if let Some(arr) = v.get("steps").and_then(|s| s.as_array()) {
        if !arr.is_empty() {
            return WorkspaceFileKind::Sequence;
        }
    }
    if v.get("request").is_some() {
        return WorkspaceFileKind::Request;
    }
    WorkspaceFileKind::Unknown
}

/// Read a single `*.json` / `*.yaml` / `*.yml` file and classify it by top-level keys (`steps` vs `request`).
pub fn classify_nativedoctor_file(path: &Path) -> Result<WorkspaceFileKind> {
    let text = std::fs::read_to_string(path)?;
    let v = parse_as_json_value(path, &text)?;
    Ok(classify_json_document(&v))
}

/// Like [`list_request_paths`], but each path is classified. Files that fail to read or parse are skipped with a warning.
pub fn list_workspace_catalog(dir: &Path) -> Result<Vec<(PathBuf, WorkspaceFileKind)>> {
    let paths = list_request_paths(dir)?;
    let mut out = Vec::with_capacity(paths.len());
    for p in paths {
        match classify_nativedoctor_file(&p) {
            Ok(kind) => out.push((p, kind)),
            Err(e) => {
                warn!(
                    path = %p.display(),
                    error = %e,
                    "workspace catalog: skipped file"
                );
            }
        }
    }
    Ok(out)
}

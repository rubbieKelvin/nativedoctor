//! Recursive workspace scanning: locate OpenAPI documents and Rhai scripts under a directory.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use tracing::debug;

fn has_extension(path: &Path, exts: &[&str]) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| exts.iter().any(|want| e.eq_ignore_ascii_case(want)))
        .unwrap_or(false)
}

/// Best-effort sniff: does this YAML/JSON file look like an OpenAPI 3.x document? Reads the whole
/// file (workspace files are small) rather than trying to parse partially.
fn looks_like_openapi_doc(path: &Path) -> bool {
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    let Ok(value) = (if has_extension(path, &["json"]) {
        serde_json::from_str::<serde_json::Value>(&text).map_err(|_| ())
    } else {
        serde_yaml::from_str::<serde_json::Value>(&text).map_err(|_| ())
    }) else {
        return false;
    };
    value
        .get("openapi")
        .and_then(|v| v.as_str())
        .map(|v| v.starts_with("3."))
        .unwrap_or(false)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>, matcher: &impl Fn(&Path) -> bool) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }
        if path.is_dir() {
            walk(&path, out, matcher);
        } else if path.is_file() && matcher(&path) {
            out.push(path);
        }
    }
}

/// Recursively find files anywhere under `root` whose contents look like an OpenAPI 3.x document.
pub fn find_openapi_docs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(root, &mut out, &|p| {
        has_extension(p, &["yaml", "yml", "json"]) && looks_like_openapi_doc(p)
    });
    let unique: BTreeSet<PathBuf> = out.into_iter().collect();
    let sorted: Vec<PathBuf> = unique.into_iter().collect();
    debug!(root = %root.display(), count = sorted.len(), "find_openapi_docs");
    sorted
}

/// Recursively find `*.rhai` scripts under `root`.
pub fn find_rhai_scripts(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk(root, &mut out, &|p| has_extension(p, &["rhai"]));
    let unique: BTreeSet<PathBuf> = out.into_iter().collect();
    unique.into_iter().collect()
}

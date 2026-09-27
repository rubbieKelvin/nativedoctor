//! Load request YAML/JSON from disk and resolve relative `post_script` paths.

use std::path::{Component, Path, PathBuf};

/// Lexically normalize `.` and `..` in `path` (no filesystem access).
///
/// Used so joined paths like `base/foo/../bar/script.rhai` match the real file location and behave
/// consistently across platforms.
pub fn normalize_path_lexical(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                out.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(out.components().next_back(), Some(Component::Normal(_))) {
                    out.pop();
                } else {
                    out.push("..");
                }
            }
            Component::Normal(_) => {
                out.push(component.as_os_str());
            }
        }
    }
    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
}

/// Join `base_dir` with the relative `file` path from the request file, then normalize.
///
/// If the resulting path exists, [`std::fs::canonicalize`] is applied so symlinks resolve; otherwise
/// the lexically normalized path is returned (for error messages and missing-file checks).
pub fn resolve_file_path(base_dir: &Path, rel: &str) -> PathBuf {
    let joined = base_dir.join(rel);
    let normalized = normalize_path_lexical(&joined);
    return std::fs::canonicalize(&normalized).unwrap_or(normalized);
}

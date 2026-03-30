//! Load request YAML/JSON from disk and resolve relative `post_script` paths.

use std::path::{Component, Path, PathBuf};

use tracing::debug;

use crate::error::{Error, Result};
use crate::model::RequestFile;

/// Read and deserialize a request file. Extension must be `.json`, `.yaml`, or `.yml`.
///
/// Returns the parsed document and the **parent directory** of `path`, used to resolve
/// [`RequestFile::post_script`] paths.
pub fn load_request_file(path: &Path) -> Result<(RequestFile, PathBuf)> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let text = std::fs::read_to_string(path)?;

    let file: RequestFile = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&text).map_err(|e| Error::ParseYaml {
            path: path.to_path_buf(),
            source: e,
        })?,
        "json" => serde_json::from_str(&text).map_err(|e| Error::ParseJson {
            path: path.to_path_buf(),
            source: e,
        })?,
        _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
    };
    let base = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    debug!(
        path = %path.display(),
        format = %ext,
        name = ?file.name,
        "loaded request file"
    );
    Ok((file, base))
}

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

/// Join `base_dir` with the relative `post_script` path from the request file, then normalize.
///
/// If the resulting path exists, [`std::fs::canonicalize`] is applied so symlinks resolve; otherwise
/// the lexically normalized path is returned (for error messages and missing-file checks).
pub fn resolve_post_script(base_dir: &Path, rel: &str) -> PathBuf {
    let joined = base_dir.join(rel);
    let normalized = normalize_path_lexical(&joined);
    std::fs::canonicalize(&normalized).unwrap_or(normalized)
}

use std::path::{Path, PathBuf};

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

/// Join `base_dir` with the relative `post_script` path from the request file.
pub fn resolve_post_script(base_dir: &Path, rel: &str) -> PathBuf {
    base_dir.join(rel)
}

use std::path::{Path, PathBuf};

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
    let file = match ext.as_str() {
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
    Ok((file, base))
}

/// Join `base_dir` with the relative `post_script` path from the request file.
pub fn resolve_post_script(base_dir: &Path, rel: &str) -> PathBuf {
    base_dir.join(rel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn resolve_post_script_joins_base() {
        assert_eq!(
            resolve_post_script(Path::new("/foo/bar"), "./x.rhai"),
            PathBuf::from("/foo/bar/x.rhai")
        );
    }

    #[test]
    fn load_yaml_file() {
        let dir = tempdir().unwrap();
        let p = dir.path().join("r.yaml");
        std::fs::write(
            &p,
            b"version: 1\nrequest:\n  method: GET\n  url: https://example.com\npost_script: ./a.rhai\n",
        )
        .unwrap();
        let (req, base) = load_request_file(&p).unwrap();
        assert_eq!(req.request.method, "GET");
        assert_eq!(req.post_script.as_deref(), Some("./a.rhai"));
        assert_eq!(base, dir.path());
    }
}

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::error::Result;

fn is_request_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| matches!(e.to_lowercase().as_str(), "json" | "yaml" | "yml"))
        .unwrap_or(false)
}

fn walk_dir(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let read = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e),
    };
    for entry in read {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            walk_dir(&p, out)?;
        } else if is_request_file(&p) {
            out.push(p);
        }
    }
    Ok(())
}

/// Recursively lists `*.json`, `*.yaml`, `*.yml` under `dir`, sorted and deduplicated.
pub fn list_request_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    walk_dir(dir, &mut paths).map_err(crate::error::Error::Io)?;
    let unique: BTreeSet<PathBuf> = paths.into_iter().collect();
    let mut sorted: Vec<PathBuf> = unique.into_iter().collect();
    sorted.sort();
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn list_finds_nested() {
        let tmp = std::env::temp_dir().join(format!("nd-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("a/b")).unwrap();
        fs::write(tmp.join("a/b/x.yaml"), "request:\n  method: GET\n  url: u\n").unwrap();
        fs::write(tmp.join("root.json"), "{\"request\":{\"method\":\"GET\",\"url\":\"u\"}}")
            .unwrap();
        let list = list_request_paths(&tmp).unwrap();
        assert_eq!(list.len(), 2);
        let _ = fs::remove_dir_all(&tmp);
    }
}

//! Captured values, saved per environment under `.nativedoctor/state/`.

use std::fs;
use std::io::Write;
use std::path::Path;

use indexmap::IndexMap;

use crate::error::{Error, Result};

/// Creates the local folder and makes it ignore itself, so its contents can't be committed.
pub fn ensure_local_dir(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).map_err(|e| Error::io(dir, e))?;
    let ignore = dir.join(".gitignore");
    if !ignore.exists() {
        fs::write(&ignore, "*\n").map_err(|e| Error::io(&ignore, e))?;
    }
    return Ok(());
}

pub fn load(path: &Path) -> Result<IndexMap<String, String>> {
    if !path.exists() {
        return Ok(IndexMap::new());
    }
    let text = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    return serde_yaml_ng::from_str(&text).map_err(|source| Error::Yaml {
        path: path.to_path_buf(),
        source,
    });
}

/// Merges `values` into the file.
pub fn save(path: &Path, values: &[(String, String)]) -> Result<()> {
    let mut merged = load(path)?;
    for (name, value) in values {
        merged.insert(name.clone(), value.clone());
    }
    let text = serde_yaml_ng::to_string(&merged).map_err(|source| Error::Yaml {
        path: path.to_path_buf(),
        source,
    })?;

    return write_private(path, text.as_bytes());
}

/// Replaces a file under `.nativedoctor/state/`, readable only by its owner.
pub fn write_private(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::Invalid(format!("{} has no parent", path.display())))?;
    fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
    if let Some(local) = parent.parent() {
        ensure_local_dir(local)?;
    }

    let temp = path.with_extension("tmp");
    let mut file = private_file(&temp)?;
    file.write_all(bytes).map_err(|e| Error::io(&temp, e))?;
    drop(file);
    return fs::rename(&temp, path).map_err(|e| Error::io(path, e));
}

#[cfg(unix)]
fn private_file(path: &Path) -> Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    return fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .map_err(|e| Error::io(path, e));
}

#[cfg(not(unix))]
fn private_file(path: &Path) -> Result<fs::File> {
    fs::File::create(path).map_err(|e| Error::io(path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_merges_and_ignores_itself() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/state/dev.yaml");
        save(&path, &[("A".into(), "1".into())]).unwrap();
        save(&path, &[("B".into(), "2".into())]).unwrap();

        let values = load(&path).unwrap();
        assert_eq!(values["A"], "1");
        assert_eq!(values["B"], "2");
        assert!(dir.path().join(".nativedoctor/.gitignore").exists());
    }

    #[cfg(unix)]
    #[test]
    fn state_is_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/state/dev.yaml");
        save(&path, &[("A".into(), "1".into())]).unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}

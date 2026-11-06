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

/// Forgets one captured value.
pub fn remove(path: &Path, name: &str) -> Result<()> {
    let mut values = load(path)?;
    if values.shift_remove(name).is_none() {
        return Ok(());
    }
    let text = serde_yaml_ng::to_string(&values).map_err(|source| Error::Yaml {
        path: path.to_path_buf(),
        source,
    })?;
    return write_private(path, text.as_bytes());
}

/// Forgets all captured values. A missing file is fine.
pub fn clear(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    return fs::remove_file(path).map_err(|e| Error::io(path, e));
}

/// Reads a secrets file. A missing file gives no values.
pub fn load_secrets(path: &Path) -> Result<IndexMap<String, String>> {
    if !path.exists() {
        return Ok(IndexMap::new());
    }
    let text = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    return parse_secrets(&text).map_err(|e| Error::Invalid(format!("{}: {e}", path.display())));
}

fn parse_secrets(text: &str) -> std::result::Result<IndexMap<String, String>, dotenvy::Error> {
    return dotenvy::from_read_iter(text.as_bytes()).collect();
}

/// Makes a secrets file hold exactly `values`, keeping comments and unrelated lines.
pub fn save_secrets(path: &Path, values: &IndexMap<String, String>) -> Result<()> {
    let original = if path.exists() {
        fs::read_to_string(path).map_err(|e| Error::io(path, e))?
    } else {
        String::new()
    };

    let mut written = Vec::new();
    let mut lines = Vec::new();
    for line in original.lines() {
        let Some(name) = assigned_name(line) else {
            lines.push(line.to_string());
            continue;
        };
        if let Some(value) = values.get(name) {
            lines.push(format!("{name}={}", quote_secret(value)));
            written.push(name.to_string());
        }
    }
    for (name, value) in values {
        if !written.contains(name) {
            lines.push(format!("{name}={}", quote_secret(value)));
        }
    }
    let text = lines.join("\n") + "\n";

    // A multi-line value in the old file can't be edited line by line, so check the result.
    let parsed = parse_secrets(&text).map_err(|e| Error::Unpreserved(e.to_string()))?;
    let matches = parsed.len() == values.len()
        && values.iter().all(|(name, value)| parsed.get(name) == Some(value));
    if !matches {
        return Err(Error::Unpreserved(format!(
            "{} has lines that can't be edited safely; change it by hand",
            path.display()
        )));
    }
    return write_private(path, text.as_bytes());
}

/// The name a `NAME=value` line sets, if that's what it is.
fn assigned_name(line: &str) -> Option<&str> {
    let line = line.trim_start();
    if line.starts_with('#') {
        return None;
    }
    let line = line.strip_prefix("export ").unwrap_or(line);
    return line.split_once('=').map(|(name, _)| name.trim());
}

/// Quotes a value so dotenv reads it back exactly, without expanding `$`.
fn quote_secret(value: &str) -> String {
    let plain = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':'));
    if plain {
        return value.to_string();
    }
    if !value.contains('\'') {
        return format!("'{value}'");
    }
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$")
        .replace('\n', "\\n");
    return format!("\"{escaped}\"");
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

    #[test]
    fn remove_and_clear_forget_values() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/state/dev.yaml");
        save(&path, &[("A".into(), "1".into()), ("B".into(), "2".into())]).unwrap();
        remove(&path, "A").unwrap();
        assert!(!load(&path).unwrap().contains_key("A"));
        clear(&path).unwrap();
        assert!(load(&path).unwrap().is_empty());
        clear(&path).unwrap();
    }

    #[test]
    fn secrets_round_trip_any_value_and_keep_comments() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/secrets/dev.env");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "# my note\nOLD=1\nKEEP=2\n").unwrap();

        let tricky = [
            "simple", "", "a b", "$HOME", "${HOME}", "it's", "say \"hi\"", "it's $HOME \\ \"x\"",
            "#hash", "a=b", "tab\there",
        ];
        let mut values = IndexMap::new();
        values.insert("KEEP".to_string(), "2".to_string());
        for (index, value) in tricky.iter().enumerate() {
            values.insert(format!("S{index}"), value.to_string());
        }
        save_secrets(&path, &values).unwrap();

        assert_eq!(load_secrets(&path).unwrap(), values);
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.starts_with("# my note\n"), "{text}");
        assert!(!text.contains("OLD"), "{text}");
    }

    #[cfg(unix)]
    #[test]
    fn secrets_are_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/secrets/dev.env");
        let values = IndexMap::from([("A".to_string(), "1".to_string())]);
        save_secrets(&path, &values).unwrap();
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
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

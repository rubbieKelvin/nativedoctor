//! Load and update `runtime.nativedoctor.json` for [`super::RuntimeEnv`].

use std::fs;
use std::path::Path;

use nd_constants::RUNTIME_PERSIST_FILENAME;
use serde_json::{Map, Value};

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

/// Converts a JSON value into the string stored in the runtime map (same rules as loading from file).
pub(crate) fn json_value_to_runtime_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_else(|_| other.to_string()),
    }
}

/// Reads the JSON object from `path` if the file exists; returns `Ok(None)` if missing.
pub(crate) fn read_runtime_persist_object(path: &Path) -> Result<Option<Map<String, Value>>> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    let v: Value = serde_json::from_str(&text).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    let obj = v.as_object().ok_or_else(|| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: "expected JSON object at top level".into(),
    })?;
    Ok(Some(obj.clone()))
}

/// Writes `map` to `path` as pretty-printed JSON.
pub(crate) fn write_runtime_persist_object(path: &Path, map: &Map<String, Value>) -> Result<()> {
    let text = serde_json::to_string_pretty(map).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    fs::write(path, text).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    })
}

/// Merges keys from the persist file into `env` (if the file exists).
pub(crate) fn merge_persist_file_into_env(env: &RuntimeEnv, path: &Path) -> Result<()> {
    let Some(map) = read_runtime_persist_object(path)? else {
        return Ok(());
    };
    for (k, v) in map {
        env.set_runtime(k, json_value_to_runtime_string(&v));
    }
    Ok(())
}

/// Updates `env`, merges `key` → string value into the persist file object, and writes the file.
pub(crate) fn persist_key_in_file(
    env: &RuntimeEnv,
    path: &Path,
    key: &str,
    value_str: &str,
) -> Result<()> {
    env.set_runtime(key, value_str);

    let mut map = match read_runtime_persist_object(path)? {
        Some(m) => m,
        None => Map::new(),
    };
    map.insert(key.to_string(), Value::String(value_str.to_string()));
    write_runtime_persist_object(path, &map)
}

/// Full path to `RUNTIME_PERSIST_FILENAME` inside `dir`.
pub(crate) fn runtime_persist_path_in_dir(dir: &Path) -> std::path::PathBuf {
    dir.join(RUNTIME_PERSIST_FILENAME)
}

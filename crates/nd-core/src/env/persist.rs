//! Load and update a runtime persistence file for [`super::RuntimeEnv`].
//!
//! Supported formats (by file extension, case-insensitive): **`.json`**, **`.yaml`**, **`.yml`**.
//! The document must be a single **object** (map) at the root; arrays and scalars are rejected.

use std::fs;
use std::path::Path;

use serde_json::{Map, Value};

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PersistFormat {
    Json,
    Yaml,
}

fn persist_format(path: &Path) -> Result<PersistFormat> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "json" => Ok(PersistFormat::Json),
        "yaml" | "yml" => Ok(PersistFormat::Yaml),
        _ => Err(Error::InvalidRuntimePersistFile {
            path: path.to_path_buf(),
            message: "persistence file must use extension .json, .yaml, or .yml".into(),
        }),
    }
}

fn parse_persist_value(text: &str, format: PersistFormat, path: &Path) -> Result<Value> {
    match format {
        PersistFormat::Json => serde_json::from_str(text).map_err(|e| Error::InvalidRuntimePersistFile {
            path: path.to_path_buf(),
            message: e.to_string(),
        }),
        PersistFormat::Yaml => serde_yaml::from_str(text).map_err(|e| Error::InvalidRuntimePersistFile {
            path: path.to_path_buf(),
            message: e.to_string(),
        }),
    }
}

/// Converts a JSON value into the string stored in the runtime map (same rules as loading from file).
pub(crate) fn json_value_to_runtime_string(v: &Value) -> String {
    return match v {
        Value::String(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_else(|_| other.to_string()),
    };
}

/// Reads the persisted object from `path` if the file exists; returns `Ok(None)` if the file is missing.
pub(crate) fn read_runtime_persist_object(path: &Path) -> Result<Option<Map<String, Value>>> {
    let format = persist_format(path)?;
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    })?;
    let v: Value = parse_persist_value(&text, format, path)?;
    let obj = v
        .as_object()
        .ok_or_else(|| Error::InvalidRuntimePersistFile {
            path: path.to_path_buf(),
            message: "expected mapping (object) at top level".into(),
        })?;
    return Ok(Some(obj.clone()));
}

/// Writes `map` to `path` using pretty JSON or YAML according to the path extension.
pub(crate) fn write_runtime_persist_object(path: &Path, map: &Map<String, Value>) -> Result<()> {
    let format = persist_format(path)?;
    let text = match format {
        PersistFormat::Json => serde_json::to_string_pretty(map).map_err(|e| {
            Error::InvalidRuntimePersistFile {
                path: path.to_path_buf(),
                message: e.to_string(),
            }
        })?,
        PersistFormat::Yaml => serde_yaml::to_string(map).map_err(|e| Error::InvalidRuntimePersistFile {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?,
    };
    let text = if text.ends_with('\n') {
        text
    } else {
        format!("{text}\n")
    };

    return fs::write(path, text).map_err(|e| Error::InvalidRuntimePersistFile {
        path: path.to_path_buf(),
        message: e.to_string(),
    });
}

/// Merges keys from the persist file into `env` (if the file exists).
pub(crate) fn merge_persist_file_into_env(env: &RuntimeEnv, path: &Path) -> Result<()> {
    let Some(map) = read_runtime_persist_object(path)? else {
        return Ok(());
    };

    for (k, v) in map {
        env.set(k, json_value_to_runtime_string(&v));
    }
    return Ok(());
}

/// Updates `env`, merges `key` → string value into the persist file object, and writes the file.
pub(crate) fn persist_key_in_file(
    env: &RuntimeEnv,
    path: &Path,
    key: &str,
    value_str: &str,
) -> Result<()> {
    env.set(key, value_str);

    let mut map = match read_runtime_persist_object(path)? {
        Some(m) => m,
        None => Map::new(),
    };
    map.insert(key.to_string(), Value::String(value_str.to_string()));
    return write_runtime_persist_object(path, &map);
}

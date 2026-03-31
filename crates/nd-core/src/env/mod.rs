//! Process and runtime variable map used for `${VAR}` expansion and Rhai `env` / `set`.
//!
//! `.env` file loading for [`RuntimeEnv::merge_env_file`] uses the [dotenvy](https://docs.rs/dotenvy) crate.
//! [`RuntimeEnv::merge_runtime_persist_file`] loads `runtime.nativedoctor.json` when present.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};

pub mod dynamic;

mod persist;

/// Merged view of process environment plus in-memory overrides (“runtime” variables).
///
/// Created with [`Self::from_process_env`], which copies all current `std::env` entries into an
/// internal map. [`Self::get`] reads that map first, then (when [`Self::fallback_to_process_env`]
/// is true) falls back to `std::env::var` if the key is missing (handles variables added to the
/// process after construction). [`Self::set_runtime`] only updates the internal map, so Rhai `set`
/// affects later template resolution within the same run without mutating the OS environment.
///
/// Use [`Self::isolated`] for an empty map with no process fallback (e.g. CLI `--no-default-system-env`).
#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    // Persistence file
    file: Option<PathBuf>,
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl RuntimeEnv {
    pub fn new() -> Self {
        return Self {
            file: None,
            inner: Arc::new(Mutex::new(HashMap::new())),
        };
    }

    pub fn with_env_files(self, paths: Vec<PathBuf>) -> Result<Self> {
        for path in paths.iter() {
            self.merge_env_file(path)?;
        }

        return Ok(self);
    }

    pub fn with_persistence(mut self, path: Option<PathBuf>) -> Result<Self> {
        self.file = path.clone();

        if let Some(path) = path {
            self.merge_runtime_persist_file(&path)?;
        }

        return Ok(self);
    }

    /// Resolve a variable: runtime map first
    pub fn get(&self, key: &str) -> Option<String> {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(v) = g.get(key) {
            return Some(v.clone());
        }

        return None;
    }

    /// Insert or update a runtime-only variable (visible to [`Self::get`] and Rhai `env()`).
    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.insert(key.into(), value.into());
    }

    pub fn clear(&self) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.clear();
    }

    /// Merge a map into the runtime map (e.g. sequence [`crate::model::SequenceFile::initial_variables`]).
    /// Later keys override earlier ones for the same key.
    pub fn merge_runtime_map(&self, vars: &HashMap<String, String>) {
        for (k, v) in vars {
            self.set(k, v);
        }
    }

    /// Stringifies `value`, updates the runtime map, and merges into `runtime.nativedoctor.json` at `path` (full file path).
    pub fn persist(&self, key: &str, value: &str) -> Result<()> {
        if let Some(file) = &self.file {
            persist::persist_key_in_file(self, &file, key, value)
        } else {
            return Err(Error::NoRuntimePersistFile {
                message: format!("Attempting to persist '{}'", key),
            });
        }
    }

    /// Merge key–value pairs from a persistence file.
    /// No-op if the file does not exist.
    pub fn merge_runtime_persist_file(&self, path: &Path) -> Result<()> {
        return persist::merge_persist_file_into_env(self, path);
    }

    /// Merge variables from a `.env` file into the runtime map
    /// Later entries override earlier ones for the same key.
    pub fn merge_env_file(&self, path: &Path) -> Result<()> {
        let iter = dotenvy::from_path_iter(path).map_err(|e| map_dotenvy_error(path, e))?;
        for item in iter {
            let (key, value) = item.map_err(|e| map_dotenvy_error(path, e))?;
            if key.is_empty() {
                return Err(Error::EnvFileParse {
                    path: path.to_path_buf(),
                    line: 0,
                    message: "empty variable name".into(),
                });
            }
            self.set(key, value);
        }
        return Ok(());
    }
}

fn map_dotenvy_error(path: &Path, err: dotenvy::Error) -> Error {
    return match err {
        dotenvy::Error::Io(source) => Error::EnvFileRead {
            path: path.to_path_buf(),
            source,
        },
        dotenvy::Error::LineParse(line, index) => Error::EnvFileParse {
            path: path.to_path_buf(),
            line: index.saturating_add(1),
            message: format!("invalid line: {line:?}"),
        },
        dotenvy::Error::EnvVar(e) => Error::EnvFileParse {
            path: path.to_path_buf(),
            line: 0,
            message: e.to_string(),
        },
        other => Error::EnvFileParse {
            path: path.to_path_buf(),
            line: 0,
            message: other.to_string(),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_env_file_parses_and_overrides() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), "# comment\nFOO=1\nBAR=\"x y\"\nFOO=2\n").unwrap();
        let env = RuntimeEnv::new();
        env.merge_env_file(tmp.path()).unwrap();
        assert_eq!(env.get("FOO").as_deref(), Some("2"));
        assert_eq!(env.get("BAR").as_deref(), Some("x y"));
    }
}

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
    inner: Arc<Mutex<HashMap<String, String>>>,
    /// When true, [`Self::get`] may consult [`std::env::var`] after the map misses.
    fallback_to_process_env: bool,
}

impl RuntimeEnv {
    /// Snapshot all current process environment variables into the writable runtime map.
    pub fn from_process_env() -> Self {
        return Self {
            inner: Arc::new(Mutex::new(std::env::vars().collect())),
            fallback_to_process_env: true,
        };
    }

    /// Empty runtime map; [`Self::get`] does not read the process environment (unless you merge
    /// files or call [`Self::set_runtime`]).
    pub fn isolated() -> Self {
        return Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            fallback_to_process_env: false,
        };
    }

    /// Build a runtime environment the same way the CLI does for `run` / `runall` / `web`:
    /// optional process snapshot (`no_default_system_env` disables it), then
    /// `runtime.nativedoctor.json` in the **current working directory**, then each `--env` file in
    /// order (later files override earlier keys).
    pub fn from_cli_options(
        no_default_system_env: bool,
        env_files: &[PathBuf],
    ) -> Result<Self> {
        let env = if no_default_system_env {
            Self::isolated()
        } else {
            Self::from_process_env()
        };

        let cwd = std::env::current_dir()?;
        env.merge_runtime_persist_dir(&cwd)?;

        for path in env_files {
            env.merge_env_file(path)?;
        }

        return Ok(env);
    }

    /// Resolve a variable: runtime map first, then optionally live process environment.
    pub fn get(&self, key: &str) -> Option<String> {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(v) = g.get(key) {
            return Some(v.clone());
        }

        drop(g);

        return if self.fallback_to_process_env {
            std::env::var(key).ok()
        } else {
            None
        };
    }

    /// Insert or update a runtime-only variable (visible to [`Self::get`] and Rhai `env()`).
    pub fn set_runtime(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.insert(key.into(), value.into());
    }

    /// Merge a map into the runtime map (e.g. sequence [`crate::model::SequenceFile::initial_variables`]).
    /// Later keys override earlier ones for the same key.
    pub fn merge_runtime_map(&self, vars: &HashMap<String, String>) {
        for (k, v) in vars {
            self.set_runtime(k, v);
        }
    }

    /// Merge key–value pairs from `runtime.nativedoctor.json` at `path` (full file path).
    /// No-op if the file does not exist.
    pub fn merge_runtime_persist_file(&self, path: &Path) -> Result<()> {
        persist::merge_persist_file_into_env(self, path)
    }

    /// Merge key–value pairs from `runtime.nativedoctor.json` in `dir`.
    /// No-op if the file does not exist.
    pub fn merge_runtime_persist_dir(&self, dir: &Path) -> Result<()> {
        persist::merge_persist_file_into_env(self, &persist::runtime_persist_path_in_dir(dir))
    }

    /// Stringifies `value`, updates the runtime map, and merges into `runtime.nativedoctor.json` at `path` (full file path).
    pub fn persist_key_to_file(&self, path: &Path, key: &str, value: &str) -> Result<()> {
        persist::persist_key_in_file(self, path, key, value)
    }

    /// Merge variables from a `.env` file into the runtime map (parsed with [dotenvy](https://docs.rs/dotenvy)).
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
            self.set_runtime(key, value);
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
        let env = RuntimeEnv::isolated();
        env.merge_env_file(tmp.path()).unwrap();
        assert_eq!(env.get("FOO").as_deref(), Some("2"));
        assert_eq!(env.get("BAR").as_deref(), Some("x y"));
    }
}

//! Process and runtime variable map used for `${VAR}` expansion and Rhai `env` / `set`.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};

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
        Self {
            inner: Arc::new(Mutex::new(std::env::vars().collect())),
            fallback_to_process_env: true,
        }
    }

    /// Empty runtime map; [`Self::get`] does not read the process environment (unless you merge
    /// files or call [`Self::set_runtime`]).
    pub fn isolated() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            fallback_to_process_env: false,
        }
    }

    /// Resolve a variable: runtime map first, then optionally live process environment.
    pub fn get(&self, key: &str) -> Option<String> {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());

        if let Some(v) = g.get(key) {
            return Some(v.clone());
        }

        drop(g);

        if self.fallback_to_process_env {
            std::env::var(key).ok()
        } else {
            None
        }
    }

    /// Insert or update a runtime-only variable (visible to [`Self::get`] and Rhai `env()`).
    pub fn set_runtime(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.insert(key.into(), value.into());
    }

    /// Merge `KEY=value` lines from a dotenv-style file into the runtime map (later lines override
    /// earlier ones for the same key). Empty lines and `#` comments are skipped.
    pub fn merge_env_file(&self, path: &Path) -> Result<()> {
        let text = std::fs::read_to_string(path).map_err(|source| Error::EnvFileRead {
            path: path.to_path_buf(),
            source,
        })?;

        for (i, raw_line) in text.lines().enumerate() {
            let line_no = i + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let line = line.strip_prefix("export ").map(str::trim).unwrap_or(line);
            let (key, value) = parse_env_key_value(line, path, line_no)?;
            if key.is_empty() {
                return Err(Error::EnvFileParse {
                    path: path.to_path_buf(),
                    line: line_no,
                    message: "empty variable name".into(),
                });
            }
            self.set_runtime(key, value);
        }
        Ok(())
    }
}

fn parse_env_key_value(line: &str, path: &Path, line_no: usize) -> Result<(String, String)> {
    let Some(eq) = line.find('=') else {
        return Err(Error::EnvFileParse {
            path: path.to_path_buf(),
            line: line_no,
            message: "expected KEY=value".into(),
        });
    };
    let key = line[..eq].trim().to_string();
    if key.is_empty() {
        return Err(Error::EnvFileParse {
            path: path.to_path_buf(),
            line: line_no,
            message: "empty variable name before '='".into(),
        });
    }
    let mut value = line[eq + 1..].trim();
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        if (bytes[0] == b'"' && bytes[value.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[value.len() - 1] == b'\'')
        {
            value = &value[1..value.len() - 1];
        }
    }
    Ok((key, value.to_string()))
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

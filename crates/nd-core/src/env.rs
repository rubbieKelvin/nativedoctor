use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Merged view of process environment plus in-memory overrides (“runtime” variables).
///
/// Created with [`Self::from_process_env`], which copies all current `std::env` entries into an
/// internal map. [`Self::get`] reads that map first, then falls back to `std::env::var` if the key
/// is missing (handles variables added to the process after construction). [`Self::set_runtime`]
/// only updates the internal map, so Rhai `set_runtime` affects later template resolution within
/// the same run without mutating the OS environment.
#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl RuntimeEnv {
    /// Snapshot all current process environment variables into the writable runtime map.
    pub fn from_process_env() -> Self {
        let map: HashMap<String, String> = std::env::vars().collect();
        Self {
            inner: Arc::new(Mutex::new(map)),
        }
    }

    /// Resolve a variable: runtime map first, then live process environment.
    pub fn get(&self, key: &str) -> Option<String> {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = g.get(key) {
            return Some(v.clone());
        }
        drop(g);
        std::env::var(key).ok()
    }

    /// Insert or update a runtime-only variable (visible to [`Self::get`] and Rhai `env()`).
    pub fn set_runtime(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.insert(key.into(), value.into());
    }
}

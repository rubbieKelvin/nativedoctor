use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Process environment seeded into a writable runtime map; lookups prefer runtime over fresh `std::env`.
#[derive(Debug, Clone)]
pub struct RuntimeEnv {
    inner: Arc<Mutex<HashMap<String, String>>>,
}

impl RuntimeEnv {
    pub fn from_process_env() -> Self {
        let map: HashMap<String, String> = std::env::vars().collect();
        Self {
            inner: Arc::new(Mutex::new(map)),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(v) = g.get(key) {
            return Some(v.clone());
        }
        drop(g);
        std::env::var(key).ok()
    }

    pub fn set_runtime(&self, key: impl Into<String>, value: impl Into<String>) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        g.insert(key.into(), value.into());
    }

    pub fn clone_arc(&self) -> Arc<Mutex<HashMap<String, String>>> {
        Arc::clone(&self.inner)
    }
}

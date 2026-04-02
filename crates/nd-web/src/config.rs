//! Shared configuration for the web server (set once before launch).

use std::path::PathBuf;
use std::sync::OnceLock;

/// Global options passed from `nativedoctor web` (mirrors CLI globals for runs).
pub struct WebConfig {
    pub root: PathBuf,
    pub env_files: Vec<PathBuf>,
    pub verbose: bool,
}

static WEB_CONFIG: OnceLock<WebConfig> = OnceLock::new();

pub fn set_web_config(cfg: WebConfig) -> Result<(), String> {
    WEB_CONFIG
        .set(cfg)
        .map_err(|_| "internal error: web config already set".to_string())
}

pub fn web_config() -> Option<&'static WebConfig> {
    WEB_CONFIG.get()
}

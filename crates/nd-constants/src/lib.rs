//! Shared literals for nativedoctor crates.

pub const USER_AGENT: &str = concat!("nativedoctor/", env!("CARGO_PKG_VERSION"));
pub const DEFAULT_ACCEPT: &str = "application/json, */*;q=0.5";

pub const MANIFEST_FILE: &str = "nativedoctor.yaml";
pub const REQUEST_SUFFIX: &str = ".request.yaml";
pub const ENVIRONMENTS_DIR: &str = "environments";
pub const LOCAL_DIR: &str = ".nativedoctor";

/// State file name used when no environment is selected.
pub const DEFAULT_STATE: &str = "_default";

pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
pub const DEFAULT_MAX_REDIRECTS: usize = 30;

pub const REDACTED: &str = "***";

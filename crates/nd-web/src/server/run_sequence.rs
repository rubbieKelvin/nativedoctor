//! Run a sequence file (same layering as `nativedoctor run -s`).

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use crate::config;
use crate::path_utils::resolve_basename_under_root;

#[server]
pub async fn run_sequence_file(name: String) -> Result<String, ServerFnError> {
    let cfg = config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let _path = resolve_basename_under_root(&cfg.root, &name)?;

    todo!();
}

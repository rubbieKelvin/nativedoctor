//! Run a single request file from the workspace.

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use crate::config;
use crate::path_utils::resolve_basename_under_root;

#[server]
pub async fn run_request_file(name: String) -> Result<String, ServerFnError> {
    let cfg = config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let _path = resolve_basename_under_root(&cfg.root, &name)?;

    todo!();
}

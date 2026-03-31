//! Run a single request file from the workspace.

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use nd_core::{
    execute_request_post_script, execute_request_with_env, OutcomePolicy, RunOptions, RuntimeEnv,
};

use crate::config;
use crate::path_utils::resolve_basename_under_root;

#[server]
pub async fn run_request_file(name: String) -> Result<String, ServerFnError> {
    let cfg = config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let path = resolve_basename_under_root(&cfg.root, &name)?;

    let env = RuntimeEnv::from_cli_options(cfg.no_default_system_env, &cfg.env_files)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let opts = RunOptions {
        verbose: cfg.verbose,
        no_post_script: false,
        dry_run: false,
        allow_error_status: true,
        outcome_policy: OutcomePolicy::SingleRequest,
    };

    let output = execute_request_with_env(&path, &opts, &env)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    execute_request_post_script(&output, &opts, &env, None)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let body_preview = String::from_utf8_lossy(&output.body);
    let body_short: String = body_preview.chars().take(4000).collect();
    let truncated = if body_preview.len() > 4000 {
        format!("{body_short}\n… (truncated)")
    } else {
        body_short.to_string()
    };

    Ok(format!(
        "HTTP {}\nURL: {}\n\n{}",
        output.status, output.final_url, truncated
    ))
}

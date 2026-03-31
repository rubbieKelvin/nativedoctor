//! Run a sequence file (same layering as `nativedoctor run -s`).

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use nd_core::{
    execute_request_post_script, execute_request_with_env, expand_hashmap_values,
    load_sequence_file, OutcomePolicy, RunOptions, RuntimeEnv,
};

use crate::config;
use crate::path_utils::resolve_basename_under_root;

#[server]
pub async fn run_sequence_file(name: String) -> Result<String, ServerFnError> {
    let cfg = config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let path = resolve_basename_under_root(&cfg.root, &name)?;

    let env = RuntimeEnv::from_cli_options(cfg.no_default_system_env, &cfg.env_files)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let (seq, base_dir) =
        load_sequence_file(&path).map_err(|e| ServerFnError::new(e.to_string()))?;

    if seq.steps.is_empty() {
        return Err(ServerFnError::new(
            "sequence must contain at least one step",
        ));
    }

    let expanded_initial_vars = expand_hashmap_values(&env, &seq.initial_variables)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    env.merge_runtime_map(&expanded_initial_vars);

    let step_opts = RunOptions {
        verbose: cfg.verbose,
        no_post_script: false,
        dry_run: false,
        allow_error_status: true,
        outcome_policy: OutcomePolicy::SequenceStep,
    };

    let mut lines = Vec::new();
    let total = seq.steps.len();

    for (i, step) in seq.steps.iter().enumerate() {
        let step_path = base_dir.join(&step.file);
        if !step_path.is_file() {
            return Err(ServerFnError::new(format!(
                "sequence step request file not found: {}",
                step_path.display()
            )));
        }

        let output = execute_request_with_env(&step_path, &step_opts, &env)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        execute_request_post_script(&output, &step_opts, &env, Some((step, base_dir.as_path())))
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let body_preview = String::from_utf8_lossy(&output.body);
        let body_short: String = body_preview.chars().take(2000).collect();
        let truncated = if body_preview.len() > 2000 {
            format!("{body_short}\n… (truncated)")
        } else {
            body_short
        };

        lines.push(format!(
            "--- step {} / {} ({}) ---\nHTTP {}\n{}\n{}",
            i + 1,
            total,
            step.file,
            output.status,
            output.final_url,
            truncated
        ));
    }

    Ok(lines.join("\n\n"))
}

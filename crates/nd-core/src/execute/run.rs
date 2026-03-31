//! Orchestrate load → expand → send → post-script → status checks.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use nd_constants::RUNTIME_PERSIST_FILENAME;
use tracing::debug;

use super::client::{build_client, merge_url_query, send_request};
use super::post_script::{run_request_post_script, run_sequence_flow_post_scripts};
use super::prepare::expand_http_request;
use super::types::{ExecutionResult, OutcomePolicy, PreparedRequest, RunOptions};
use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::load::load_request_file;
use crate::model::SequenceStep;
use crate::RequestFile;

/// Path to `runtime.nativedoctor.json` in the process current working directory.
fn runtime_persist_file() -> Result<PathBuf> {
    let cwd = std::env::current_dir().map_err(Error::Io)?;
    Ok(cwd.join(RUNTIME_PERSIST_FILENAME))
}

/// Load → expand with `env` → send (unless dry-run) → Rhai / status handling per [`RunOptions::outcome_policy`].
///
/// [`OutcomePolicy::SingleRequest`]: post-script runs before failing on HTTP ≥ 400 (unless
/// `allow_error_status`). [`OutcomePolicy::SequenceStep`]: without a request `post_script` or
/// sequence `post_scripts` on the step, fail on HTTP ≥ 400 when `allow_error_status` is false;
/// otherwise run Rhai and do not fail on HTTP status alone.
pub async fn execute_request_with_env(
    path: &Path,
    opts: &RunOptions,
    env: &RuntimeEnv,
) -> Result<ExecutionResult> {
    let (doc, base_dir) = load_request_file(path)?;
    let prep = expand_http_request(env, &doc.request)?;

    debug!(
        path = %path.display(),
        request_name = ?doc.name,
        dry_run = opts.dry_run,
        outcome_policy = ?opts.outcome_policy,
        "execute_request_with_env"
    );

    // If this is a dry run (ie. no IO, let's skip actually calling the request)
    if opts.dry_run {
        debug!(path = %path.display(), "dry_run: skipping HTTP");
        return Ok(dry_run_result(&prep, &doc));
    }

    let client = build_client(&doc.request)?;
    let start = Instant::now();

    let response = send_request(&client, &prep).await?;
    let duration = start.elapsed();
    let status = response.status().as_u16();
    let final_url = response.url().to_string();
    let mut resp_headers = Vec::new();

    for (name, value) in response.headers().iter() {
        if let Ok(s) = value.to_str() {
            resp_headers.push((name.as_str().to_string(), s.to_string()));
        }
    }

    let body = response.bytes().await.map_err(Error::Http)?.to_vec();

    debug!(
        status,
        final_url = %final_url,
        ?duration,
        body_len = body.len(),
        "HTTP response received"
    );

    return Ok(ExecutionResult {
        method: prep.method.clone(),
        request_name: doc.name.clone(),
        status,
        final_url,
        headers: resp_headers,
        body,
        duration,
        base_dir,
        doc,
    });
}

/// Runs the request file’s optional `post_script`, then optional sequence-level `post_scripts` on
/// the step when `sequence_step` is `Some`.
///
/// Pass **`None`** for `sequence_step` when not running a sequence step (single-request runs).
pub fn execute_request_post_script(
    output: &ExecutionResult,
    opts: &RunOptions,
    env: &RuntimeEnv,
    sequence_step: Option<(&SequenceStep, &Path)>,
) -> Result<()> {
    let has_request_script = output.doc.post_script.is_some() && !opts.no_post_script;
    let has_flow_scripts = sequence_step
        .map(|(s, _)| !s.post_scripts.is_empty() && !opts.no_post_script)
        .unwrap_or(false);

    let persist_file = Some(runtime_persist_file()?);

    match opts.outcome_policy {
        OutcomePolicy::SingleRequest => {
            run_request_post_script(
                &output.doc,
                &output.base_dir,
                env,
                opts,
                output.status,
                &output.headers,
                &output.body,
                persist_file.clone(),
            )?;

            if !opts.allow_error_status && output.status >= 400 {
                return Err(Error::InvalidRequest(format!(
                    "HTTP status {} (use --allow-error-status to accept)",
                    output.status
                )));
            }
        }
        OutcomePolicy::SequenceStep => {
            if !opts.allow_error_status
                && output.status >= 400
                && !has_request_script
                && !has_flow_scripts
            {
                return Err(Error::InvalidRequest(format!(
                    "sequence step HTTP status {} (no request post_script or sequence post_scripts to handle error)",
                    output.status
                )));
            }

            if has_request_script {
                run_request_post_script(
                    &output.doc,
                    &output.base_dir,
                    env,
                    opts,
                    output.status,
                    &output.headers,
                    &output.body,
                    persist_file.clone(),
                )?;
            }

            if let Some((step, seq_base)) = sequence_step {
                run_sequence_flow_post_scripts(
                    step,
                    seq_base,
                    env,
                    opts,
                    output.status,
                    &output.headers,
                    &output.body,
                    persist_file.clone(),
                )?;
            }
        }
    }

    return Ok(());
}

/// Build a synthetic “result” for dry-run: no network, status 0, body = request body bytes.
fn dry_run_result(prep: &PreparedRequest, doc: &RequestFile) -> ExecutionResult {
    let request_name = doc.clone().name;
    let full_url = merge_url_query(&prep.url, &prep.query).unwrap_or_else(|_| prep.url.clone());
    ExecutionResult {
        method: prep.method.clone(),
        request_name,
        status: 0,
        final_url: full_url,
        headers: prep.headers.clone(),
        body: prep.body.clone().unwrap_or_default(),
        duration: Duration::ZERO,
        base_dir: PathBuf::new(),
        doc: doc.clone(),
    }
}

/// Load and expand one request file using a fresh environment.
pub fn prepare_request_file(path: &Path) -> Result<(PreparedRequest, std::path::PathBuf)> {
    let env = RuntimeEnv::from_process_env();
    return prepare_request_with_env(path, &env);
}

/// Load and expand one request file with an existing [`RuntimeEnv`] (e.g. shared sequence session).
pub fn prepare_request_with_env(
    path: &Path,
    env: &RuntimeEnv,
) -> Result<(PreparedRequest, std::path::PathBuf)> {
    let (doc, base_dir) = load_request_file(path)?;
    let prep = expand_http_request(env, &doc.request)?;
    return Ok((prep, base_dir));
}

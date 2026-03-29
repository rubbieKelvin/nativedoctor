//! Orchestrate load → expand → send → post-script → status checks.

use std::path::Path;
use std::time::{Duration, Instant};

use tracing::debug;

use super::client::{build_client, merge_url_query, send_request};
use super::post_script::run_request_post_script;
use super::prepare::expand_http_request;
use super::types::{ExecutionResult, OutcomePolicy, PreparedRequest, RunOptions};
use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::load::load_request_file;

/// Same as [`execute_request_with_env`] with a fresh [`RuntimeEnv::from_process_env`].
pub async fn execute_request_file(path: &Path, opts: RunOptions) -> Result<ExecutionResult> {
    let env = RuntimeEnv::from_process_env();
    execute_request_with_env(path, &opts, &env).await
}

/// Load → expand with `env` → send (unless dry-run) → Rhai / status handling per [`RunOptions::outcome_policy`].
///
/// [`OutcomePolicy::SingleRequest`]: post-script runs before failing on HTTP ≥ 400 (unless
/// `allow_error_status`). [`OutcomePolicy::SequenceStep`]: without an active post-script, fail on
/// HTTP ≥ 400 when `allow_error_status` is false; with post-script, run Rhai and never fail on HTTP
/// status alone.
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

    if opts.dry_run {
        debug!(path = %path.display(), "dry_run: skipping HTTP");
        return Ok(dry_run_result(&prep, doc.name.clone()));
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

    let has_active_script = doc.post_script.is_some() && !opts.no_post_script;

    match opts.outcome_policy {
        OutcomePolicy::SingleRequest => {
            run_request_post_script(&doc, &base_dir, env, opts, status, &resp_headers, &body)?;
            if !opts.allow_error_status && status >= 400 {
                return Err(Error::InvalidRequest(format!(
                    "HTTP status {status} (use --allow-error-status to accept)"
                )));
            }
        }
        OutcomePolicy::SequenceStep => {
            if has_active_script {
                run_request_post_script(&doc, &base_dir, env, opts, status, &resp_headers, &body)?;
            } else if !opts.allow_error_status && status >= 400 {
                return Err(Error::InvalidRequest(format!(
                    "sequence step HTTP status {status} (no post_script to handle error)"
                )));
            }
        }
    }

    Ok(ExecutionResult {
        method: prep.method.clone(),
        request_name: doc.name.clone(),
        status,
        final_url,
        headers: resp_headers,
        body,
        duration,
    })
}

/// Build a synthetic “result” for dry-run: no network, status 0, body = request body bytes.
fn dry_run_result(prep: &PreparedRequest, request_name: Option<String>) -> ExecutionResult {
    let full_url = merge_url_query(&prep.url, &prep.query).unwrap_or_else(|_| prep.url.clone());
    ExecutionResult {
        method: prep.method.clone(),
        request_name,
        status: 0,
        final_url: full_url,
        headers: prep.headers.clone(),
        body: prep.body.clone().unwrap_or_default(),
        duration: Duration::ZERO,
    }
}

/// Load and expand one request file using a fresh environment.
pub fn prepare_request_file(path: &Path) -> Result<(PreparedRequest, std::path::PathBuf)> {
    let env = RuntimeEnv::from_process_env();
    prepare_request_with_env(path, &env)
}

/// Load and expand one request file with an existing [`RuntimeEnv`] (e.g. shared sequence session).
pub fn prepare_request_with_env(
    path: &Path,
    env: &RuntimeEnv,
) -> Result<(PreparedRequest, std::path::PathBuf)> {
    let (doc, base_dir) = load_request_file(path)?;
    let prep = expand_http_request(env, &doc.request)?;
    Ok((prep, base_dir))
}

//! HTTP execution: expand templates, build a [`reqwest::Client`], send, then optional Rhai post-script.

use std::path::Path;
use std::time::{Duration, Instant};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{redirect, Client, Method, Url};
use tracing::debug;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::load::{load_request_file, resolve_post_script};
use crate::model::{HttpRequestSpec, RequestBody, RequestFile};
use crate::rhai_host::run_post_script;
use crate::template::{expand_json_value, expand_string};

/// How HTTP status and Rhai interact after a response (single `run` vs [`crate::sequence`] step).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutcomePolicy {
    /// Post-script runs before failing on HTTP ≥ 400 (unless [`RunOptions::allow_error_status`]).
    #[default]
    SingleRequest,
    /// Sequence rules: without a post-script, HTTP ≥ 400 fails the step; with a post-script, Rhai
    /// runs and only Rhai failure fails — HTTP status alone never fails the step.
    SequenceStep,
}

/// Options for [`execute_request_file`] / [`execute_request_with_env`] (mirrors CLI flags).
#[derive(Debug, Clone)]
pub struct RunOptions {
    pub verbose: bool,
    /// Skip `post_script` even if the request file defines one.
    pub no_post_script: bool,
    /// If true, returns immediately without I/O using a synthetic [`ExecutionResult`] (status 0).
    pub dry_run: bool,
    /// If false, status codes ≥ 400 become [`Error::InvalidRequest`] after the post-script runs
    /// ([`OutcomePolicy::SingleRequest`] only, or sequence steps **without** a post-script).
    pub allow_error_status: bool,
    pub outcome_policy: OutcomePolicy,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            verbose: false,
            no_post_script: false,
            dry_run: false,
            allow_error_status: false,
            outcome_policy: OutcomePolicy::default(),
        }
    }
}

/// Outcome of a real HTTP call (or a synthetic row for dry-run — see field docs).
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub method: Method,
    /// From the request file’s optional `name` field.
    pub request_name: Option<String>,
    /// Response status, or `0` for dry-run.
    pub status: u16,
    /// Final URL after redirects, or the expanded request URL for dry-run.
    pub final_url: String,
    /// Response headers from the wire, or request headers for dry-run.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// Time to receive the full response; zero for dry-run.
    pub duration: Duration,
}

/// Fully expanded, ready-to-send request (templates applied).
pub struct PreparedRequest {
    pub method: Method,
    pub url: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    /// If the body is set and `Content-Type` is absent, this value is sent as the header.
    pub content_type_if_missing: Option<String>,
    pub timeout_secs: u64,
    pub follow_redirects: bool,
    pub verify_tls: bool,
}

fn expand_http_request(env: &RuntimeEnv, spec: &HttpRequestSpec) -> Result<PreparedRequest> {
    let method = Method::from_bytes(spec.method.to_uppercase().as_bytes())
        .map_err(|_| Error::InvalidRequest(format!("unsupported HTTP method: {}", spec.method)))?;
    let url = expand_string(env, &spec.url)?;
    let mut query = Vec::new();
    for (k, v) in &spec.query {
        query.push((expand_string(env, k)?, expand_string(env, v)?));
    }
    let mut headers = Vec::new();
    for (k, v) in &spec.headers {
        headers.push((expand_string(env, k)?, expand_string(env, v)?));
    }
    let (body, content_type_if_missing) = match &spec.body {
        None => (None, None),
        Some(RequestBody::Text(t)) => (
            Some(expand_string(env, t)?.into_bytes()),
            Some("text/plain; charset=utf-8".to_string()),
        ),
        Some(RequestBody::Json(v)) => {
            let expanded = expand_json_value(env, v)?;
            let bytes = serde_json::to_vec(&expanded).map_err(|e| {
                Error::InvalidRequest(format!("failed to serialize JSON body: {e}"))
            })?;
            (Some(bytes), Some("application/json".to_string()))
        }
    };
    let timeout_secs = spec
        .timeout_secs
        .unwrap_or(RequestFile::default_timeout_secs());
    Ok(PreparedRequest {
        method,
        url,
        query,
        headers,
        body,
        content_type_if_missing,
        timeout_secs,
        follow_redirects: spec.follow_redirects,
        verify_tls: spec.verify_tls,
    })
}

fn build_client(spec: &HttpRequestSpec) -> Result<Client> {
    let timeout_secs = spec
        .timeout_secs
        .unwrap_or(RequestFile::default_timeout_secs());
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .redirect(if spec.follow_redirects {
            redirect::Policy::default()
        } else {
            redirect::Policy::none()
        })
        .danger_accept_invalid_certs(!spec.verify_tls)
        .build()
        .map_err(Error::Http)
}

fn merge_url_query(base: &str, query: &[(String, String)]) -> Result<String> {
    if query.is_empty() {
        return Ok(base.to_string());
    }
    let mut url = Url::parse(base)
        .map_err(|e| Error::InvalidRequest(format!("invalid URL after expansion: {base}: {e}")))?;
    for (k, v) in query {
        url.query_pairs_mut().append_pair(k, v);
    }
    Ok(url.to_string())
}

fn header_map(pairs: &[(String, String)]) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    for (k, v) in pairs {
        let name = HeaderName::from_bytes(k.as_bytes())
            .map_err(|_| Error::InvalidRequest(format!("invalid header name: {k}")))?;
        let value = HeaderValue::from_str(v)
            .map_err(|_| Error::InvalidRequest(format!("invalid header value for {k}")))?;
        map.insert(name, value);
    }
    Ok(map)
}

async fn send_request(client: &Client, prep: &PreparedRequest) -> Result<reqwest::Response> {
    let full_url = merge_url_query(&prep.url, &prep.query)?;
    debug!(
        method = %prep.method,
        url = %full_url,
        body_len = prep.body.as_ref().map(|b| b.len()).unwrap_or(0),
        "sending HTTP request"
    );
    let mut req = client.request(prep.method.clone(), &full_url);
    let mut hdrs = header_map(&prep.headers)?;
    if prep.body.is_some() {
        let has_ct = hdrs.contains_key(reqwest::header::CONTENT_TYPE);
        if !has_ct {
            if let Some(ct) = &prep.content_type_if_missing {
                let hv = HeaderValue::from_str(ct).map_err(|_| {
                    Error::InvalidRequest(format!("invalid default Content-Type: {ct}"))
                })?;
                hdrs.insert(reqwest::header::CONTENT_TYPE, hv);
            }
        }
    }
    req = req.headers(hdrs);
    if let Some(b) = &prep.body {
        req = req.body(b.clone());
    }
    req.send().await.map_err(Error::Http)
}

fn run_request_post_script(
    doc: &RequestFile,
    base_dir: &Path,
    env: &RuntimeEnv,
    opts: &RunOptions,
    status: u16,
    resp_headers: &[(String, String)],
    body: &[u8],
) -> Result<()> {
    if let Some(rel) = &doc.post_script {
        if !opts.no_post_script {
            let script_path = resolve_post_script(base_dir, rel);
            if !script_path.is_file() {
                return Err(Error::PostScriptNotFound(script_path));
            }
            debug!(
                script = %script_path.display(),
                http_status = status,
                "running post_script"
            );
            run_post_script(&script_path, env, status, resp_headers, body)?;
        }
    }
    Ok(())
}

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

/// Multiline string: request line, headers, blank line, optional body (UTF-8 or “binary” placeholder).
pub fn format_prepared_request(prep: &PreparedRequest) -> Result<String> {
    let url = merge_url_query(&prep.url, &prep.query)?;
    let mut s = format!("{} {}\n", prep.method, url);
    for (k, v) in &prep.headers {
        s.push_str(&format!("{k}: {v}\n"));
    }
    if let Some(b) = &prep.body {
        s.push('\n');
        if let Ok(txt) = std::str::from_utf8(b) {
            s.push_str(txt);
        } else {
            s.push_str(&format!("<{} bytes binary>", b.len()));
        }
        s.push('\n');
    }
    Ok(s)
}

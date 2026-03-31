//! Types for HTTP execution: options, outcomes, and expanded requests.

use std::{path::PathBuf, time::Duration};

use reqwest::Method;

use crate::model::request::RequestFile;

// /// Options for [`execute_request_file`](crate::execute::execute_request_file) /
// /// [`execute_request_with_env`](crate::execute::execute_request_with_env) (mirrors CLI flags).
// #[derive(Debug, Clone, Default)]
// pub struct RunOptions {
//     pub verbose: bool,
//     /// Skip `post_script` even if the request file defines one.
//     pub no_post_script: bool,
//     /// If true, returns immediately without I/O using a synthetic [`ExecutionResult`] (status 0).
//     pub dry_run: bool,
//     /// If false, status codes ≥ 400 become [`crate::Error::InvalidRequest`] after the post-script runs
//     /// ([`OutcomePolicy::SingleRequest`] only, or sequence steps **without** a post-script).
//     pub allow_error_status: bool,
// }

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
    /// The base directory the execution stemed from.
    pub base_dir: PathBuf,
    pub doc: RequestFile,
}

/// Fully expanded, ready-to-send request (templates applied).
pub struct PreparedRequest {
    pub method: Method,
    pub url: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    pub timeout_secs: u64,
    pub follow_redirects: bool,
    pub verify_tls: bool,
}

//! Types for HTTP execution: options, outcomes, and expanded requests.

use std::{path::PathBuf, time::Duration};

use colored::Colorize;
use reqwest::Method;

use crate::model::request::RequestFile;

pub enum PrintOptions {
    Compact,
    Normal,
    Verbose,
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
    pub doc: RequestFile,
    /// The script that triggered the call to this request
    pub initiator_script: Option<PathBuf>,
}

impl ExecutionResult {
    pub fn print(&self, style: PrintOptions) {
        let label = self.request_name.as_deref().unwrap_or_default();

        match style {
            PrintOptions::Compact => {
                println!(
                    "[{}・{}] {} ({:?})",
                    self.method.as_str().red(),
                    self.final_url,
                    self.status,
                    self.duration
                );
            }
            PrintOptions::Normal | PrintOptions::Verbose => {
                println!(
                    "{}{} {} -> {} ({:?})",
                    self.method, label, self.final_url, self.status, self.duration
                );

                if matches!(style, PrintOptions::Verbose) {
                    let hdrs = &self
                        .headers
                        .iter()
                        .map(|(k, v)| {
                            if k.eq_ignore_ascii_case("authorization") {
                                (k.clone(), "<redacted>".to_string())
                            } else {
                                (k.clone(), v.clone())
                            }
                        })
                        .collect::<Vec<(String, String)>>();

                    for (k, v) in hdrs {
                        println!("{k}: {v}");
                    }

                    println!();
                }

                let body = &self.body;

                if body.is_empty() {
                    return;
                }

                if let Ok(text) = std::str::from_utf8(body) {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&v).unwrap_or_else(|_| text.to_string())
                        );
                    } else {
                        print!("{text}");
                        if !text.ends_with('\n') {
                            println!();
                        }
                    }
                } else {
                    println!("<{} bytes binary>", body.len());
                }
            }
        };
    }
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

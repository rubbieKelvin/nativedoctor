use std::{path::PathBuf, process::exit};

use colored::Colorize;
use reqwest::{Client, Version};
use rustle_api::{
    executor::runner::{Runner, ScriptEngine},
    scripting::rhai::RhaiScripting,
};
use tracing::{error, warn};

use super::utils::{quiet_or_print, resolve_path};
use crate::ds::RunMode;

const BODY_SIZE_LIMIT: u32 = 1500000;

async fn print_response(response: reqwest::Response, quiet: bool) {
    // Print status line
    let status = response.status();
    let status_color = if status.is_success() {
        status.to_string().green()
    } else {
        status.to_string().red()
    };

    let http_version = match response.version() {
        Version::HTTP_09 => "HTTP/0.9",
        Version::HTTP_10 => "HTTP/1.0",
        Version::HTTP_11 => "HTTP/1.1",
        Version::HTTP_2 => "HTTP/2",
        Version::HTTP_3 => "HTTP/3",
        _ => "HTTP/1.1",
    };

    quiet_or_print(format!(" * {} {}", http_version, status_color,), quiet);

    // Print headers
    let headers = response.headers();
    for (name, value) in headers {
        quiet_or_print(
            format!(
                " * {}: {}",
                name.to_string().cyan(),
                value.to_str().unwrap_or("")
            ),
            quiet,
        );
    }

    // Checkout the content lenght
    let content_length = match headers.get("content-length") {
        Some(cl) => {
            let val = cl.to_str().unwrap();
            match val.parse() {
                Ok(v) => v,
                Err(_) => {
                    warn!("Could not convert {val} to i32");
                    0
                }
            }
        }
        None => 0,
    };

    if content_length > BODY_SIZE_LIMIT {
        // Redact
        quiet_or_print(
            format!(
                " * {} <preview unavailable, response is above {} limit>",
                "[BODY]".blue(),
                &BODY_SIZE_LIMIT
            ),
            quiet,
        );
    } else {
        // Print body
        if let Ok(text) = response.text().await {
            if !text.is_empty() {
                // Try to parse as JSON for pretty printing
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                    quiet_or_print(
                        format!(" * {}", serde_json::to_string_pretty(&json).unwrap()),
                        quiet,
                    );
                } else {
                    quiet_or_print(format!(" * {}", text), quiet);
                }
            }
        }
    }

    quiet_or_print(format!(""), quiet); // Empty line after response
}

async fn base_run_request(runner: &mut Runner, client: &Client, name: String, quiet: bool) {
    let stack = match runner.generate_call_queue(&name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating call stack: {}", e);
            exit(1);
        }
    };

    for request in stack {
        let schema = runner.get_request_schema(&request).unwrap();

        if request == name {
            quiet_or_print(format!("-> REQ {} {}", request.green(), schema.url), quiet);
        } else {
            quiet_or_print(
                format!(
                    "-> [{}] REQ {} {}",
                    format!("dependency/{}", name).on_yellow().black(),
                    request.green(),
                    schema.url
                ),
                quiet,
            );
        }

        let result = match runner.call_request(request, &client).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                // error!("{e}");
                exit(-1);
            }
        };

        print_response(result, quiet).await;
    }
}

async fn run_request(runner: &mut Runner, name: String, quiet: bool) {
    let client = reqwest::Client::new();
    base_run_request(runner, &client, name, quiet).await
}

async fn run_sequence(runner: &mut Runner, name: String, quiet: bool) {
    quiet_or_print(format!("SEQ {}", name.yellow()), quiet);

    let client = reqwest::Client::new();
    let seq = runner.get_sequence(&name).unwrap();

    for request in seq {
        base_run_request(runner, &client, request, quiet).await
    }
}

pub async fn run(filepath: &PathBuf, env: Option<String>, mode: RunMode, quiet: bool) {
    let (path, is_project) = resolve_path(filepath);

    // initiate the runner
    let mut runner = match Runner::new(
        &path,
        env,
        ScriptEngine::Rhai(RhaiScripting::new()),
        is_project,
    ) {
        Ok(runner) => runner,
        Err(e) => {
            eprintln!("Error creating runner: {}", e.to_string());
            exit(1);
        }
    };

    match mode {
        RunMode::Request(name) => {
            run_request(&mut runner, name, quiet).await;
        }
        RunMode::Sequence(name) => {
            run_sequence(&mut runner, name, quiet).await;
        }
        RunMode::All => {
            let seq = &runner
                .schema
                .calls
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<String>>();

            match seq.len() {
                0 => {
                    eprintln!("{}", "No defined sequence in the list".red());
                    exit(1);
                }
                1 => {
                    run_sequence(&mut runner, seq.get(0).unwrap().clone(), quiet).await;
                }
                _ => {
                    eprintln!("Specify sequence name if there's more than one in the schema");
                    exit(1);
                }
            }
        }
    };
}

//! CLI entry for **nativedoctor**: `run` / shorthand file path, `list`, and shared flags.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use nd_core::{
    execute_request_file, format_prepared_request, list_request_paths, prepare_request_file,
    ExecutionResult, RunOptions,
};

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "File-based API request runner (JSON/YAML) with optional Rhai post-scripts.")]
#[command(args_conflicts_with_subcommands = false)]
struct Cli {
    /// Log extra detail (full request before send, response headers on stdout).
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Do not run `post_script` from the request file.
    #[arg(long, global = true)]
    no_post_script: bool,

    /// Expand and print the request only; no network I/O.
    #[arg(long, global = true)]
    dry_run: bool,

    /// Treat HTTP 4xx/5xx as success for exit status (post-script still runs first).
    #[arg(long, global = true)]
    allow_error_status: bool,

    #[command(subcommand)]
    command: Option<Command>,

    /// If no subcommand is given, this file is executed (same as `run <FILE>`).
    #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Run one request definition.
    Run {
        path: PathBuf,
    },
    /// Print all `*.json` / `*.yaml` / `*.yml` paths under a directory (recursive, sorted).
    List {
        dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> std::result::Result<(), String> {
    let run_opts = || RunOptions {
        verbose: cli.verbose,
        no_post_script: cli.no_post_script,
        dry_run: cli.dry_run,
        allow_error_status: cli.allow_error_status,
    };

    match &cli.command {
        Some(Command::List { dir }) => {
            let paths = list_request_paths(dir).map_err(|e| e.to_string())?;
            if paths.is_empty() {
                println!("(no request files found)");
            } else {
                for p in paths {
                    println!("{}", p.display());
                }
            }
        }
        Some(Command::Run { path }) => {
            run_one(path, &cli, run_opts()).await?;
        }
        None => {
            let path = cli
                .file
                .as_ref()
                .ok_or_else(|| "expected a subcommand or a request file path".to_string())?;
            run_one(path, &cli, run_opts()).await?;
        }
    }
    Ok(())
}

/// Run or dry-run a single request file; prints human-readable output to stdout/stderr.
async fn run_one(
    path: &std::path::Path,
    cli: &Cli,
    opts: RunOptions,
) -> std::result::Result<(), String> {
    if opts.dry_run {
        let (prep, _) = prepare_request_file(path).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        print!("{summary}");
        return Ok(());
    }
    if cli.verbose {
        // Second load: keeps `nd-core` free of `eprintln!` while still showing the resolved request.
        let (prep, _) = prepare_request_file(path).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        eprint!("--- request ---\n{summary}\n--- response ---\n");
    }
    let result = execute_request_file(path, opts)
        .await
        .map_err(|e| e.to_string())?;
    print_result(&result, cli.verbose)?;
    Ok(())
}

fn redact_headers(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| {
            if k.eq_ignore_ascii_case("authorization") {
                (k.clone(), "<redacted>".to_string())
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

/// Status line always; verbose adds headers; body is pretty-printed JSON when valid UTF-8 JSON.
fn print_result(result: &ExecutionResult, verbose: bool) -> std::result::Result<(), String> {
    println!(
        "{} {} -> {} ({:?})",
        result.method, result.final_url, result.status, result.duration
    );
    if verbose {
        let hdrs = redact_headers(&result.headers);
        for (k, v) in hdrs {
            println!("{k}: {v}");
        }
        println!();
    }
    let body = &result.body;
    if body.is_empty() {
        return Ok(());
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
    Ok(())
}

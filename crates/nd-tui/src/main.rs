//! Standalone `nd-tui` binary (primary entry is `nativedoctor tui` from `nd-cli`).

use std::path::PathBuf;

use clap::{ArgAction, Parser};
use nd_tui::{run_tui, TuiOptions};

#[derive(Parser)]
#[command(name = "nd-tui")]
#[command(about = "Terminal UI for running nativedoctor request files and Rhai scripts.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Load `KEY=value` pairs from each dotenv-style file (later files override earlier).
    #[arg(long, value_name = "FILE", action = ArgAction::Append)]
    env: Vec<PathBuf>,

    /// JSON or YAML file for Rhai `persist()` and runtime merge.
    #[arg(long, value_name = "FILE")]
    persistence_file: Option<PathBuf>,

    /// Expand requests only; no network I/O.
    #[arg(long)]
    no_network_io: bool,

    /// Extra detail in the request preview path.
    #[arg(short, long)]
    verbose: bool,

    /// Reuse runtime variables across multiple files.
    #[arg(long)]
    retain_runtime: bool,

    /// Disable streaming HTTP response chunks (buffer full body before display).
    #[arg(long = "no-stream", action = ArgAction::SetTrue)]
    no_stream: bool,

    /// Request files (`*.json`, `*.yaml`, `*.yml`) or Rhai scripts (`*.rhai`).
    #[arg(value_name = "FILE", required = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let args = Args::parse();
    let stream = !args.no_stream;
    let r = run_tui(TuiOptions {
        verbose: args.verbose,
        no_network_io: args.no_network_io,
        retain_runtime: args.retain_runtime,
        paths: args.paths,
        persistence_file: args.persistence_file,
        env_files: args.env,
        stream,
    })
    .await;

    match r {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}

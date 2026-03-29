//! CLI entry for **nativedoctor**: `run` / shorthand file path, `list`, `sequence`, `new`, and shared flags.

mod cmd_generate;
mod cmd_new;
mod cmd_run;
mod logging;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use nd_core::list_request_paths;

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "File-based API request runner (JSON/YAML) with optional Rhai post-scripts.")]
#[command(args_conflicts_with_subcommands = false)]
pub(crate) struct Cli {
    /// Log extra detail (full request before send, response headers on stdout) and enable
    /// `nd_core=debug` tracing unless `RUST_LOG` is set.
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

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum GenerateFormat {
    #[default]
    Yaml,
    Json,
}

impl From<GenerateFormat> for nd_generate::OutputFormat {
    fn from(f: GenerateFormat) -> Self {
        match f {
            GenerateFormat::Yaml => Self::Yaml,
            GenerateFormat::Json => Self::Json,
        }
    }
}

#[derive(Subcommand)]
enum Command {
    /// Run one request definition.
    Run { path: PathBuf },
    /// Run an ordered list of request files with one shared runtime environment (see sequence JSON/YAML).
    Sequence { path: PathBuf },
    /// List `*.json` / `*.yaml` / `*.yml` in a directory (immediate children only, sorted).
    List { dir: PathBuf },
    /// Generate nativedoctor request files from an OpenAPI 3.0.x document (JSON or YAML).
    Generate {
        /// OpenAPI spec file (.json, .yaml, or .yml).
        #[arg(short = 'i', long = "input", value_name = "FILE")]
        input: PathBuf,
        /// Output directory (created if missing).
        #[arg(short = 'o', long = "output", value_name = "DIR")]
        output: PathBuf,
        /// Request file format for generated files.
        #[arg(long, value_enum, default_value_t = GenerateFormat::Yaml)]
        format: GenerateFormat,
    },
    /// Create a starter request or sequence file (e.g. `new --sequence seq-a.json`).
    New {
        /// Write a default sequence file (.json, .yaml, or .yml).
        #[arg(long, short = 's', value_name = "PATH", conflicts_with = "request")]
        sequence: Option<PathBuf>,
        /// Write a default request file (.json, .yaml, or .yml).
        #[arg(long, short = 'r', value_name = "PATH", conflicts_with = "sequence")]
        request: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    logging::init(cli.verbose);
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(cli: Cli) -> std::result::Result<(), String> {
    match &cli.command {
        Some(Command::Generate {
            input,
            output,
            format,
        }) => {
            cmd_generate::run_generate(input, output, (*format).into())?;
        }
        Some(Command::New { sequence, request }) => {
            cmd_new::run_new(sequence.as_ref(), request.as_ref())?;
        }
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
            cmd_run::run_one(path, &cli, cmd_run::run_opts(&cli)).await?;
        }
        Some(Command::Sequence { path }) => {
            cmd_run::run_sequence(path, &cli, cmd_run::run_opts(&cli)).await?;
        }
        None => {
            let path = cli
                .file
                .as_ref()
                .ok_or_else(|| "expected a subcommand or a request file path".to_string())?;
            cmd_run::run_one(path, &cli, cmd_run::run_opts(&cli)).await?;
        }
    }
    Ok(())
}

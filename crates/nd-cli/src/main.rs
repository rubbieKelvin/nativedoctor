//! CLI entry for **nativedoctor**: `run`, `runall`, shorthand file path, `list`, `new`, and shared flags.

mod cmd_generate;
mod cmd_new;
mod cmd_run;
mod cmd_runall;
mod logging;
mod print;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use nd_core::list_request_paths;

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "File-based API request runner (JSON/YAML) with optional Rhai post-scripts.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "rubbie kelvin <dev.rubbie@gmail.com>")]
#[command(args_conflicts_with_subcommands = false)]
pub(crate) struct Cli {
    /// Load `KEY=value` pairs from each dotenv-style file into the runtime (later files override earlier).
    #[arg(long = "env", value_name = "FILE", global = true, action = ArgAction::Append)]
    env: Vec<PathBuf>,

    /// Do not seed the runtime map from the current process environment (only `--env` files).
    #[arg(long, global = true)]
    no_default_system_env: bool,

    /// Log extra detail and enable `nd_core=debug` tracing unless `RUST_LOG` is set.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Command>,

    /// If no subcommand is given, this file is executed as a single request (same as `run <FILE>`).
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
    /// Run one request file, or a sequence file when `--sequence` is set.
    Run {
        /// Treat `FILE` as a sequence (ordered steps, one shared runtime environment).
        #[arg(long, short = 's')]
        sequence: bool,
        /// Do not run `post_script` from the request or sequence file.
        #[arg(long)]
        no_post: bool,
        /// Expand and print the request only; no network I/O (no request is actually run).
        #[arg(long)]
        dry_run: bool,
        /// Treat HTTP 4xx/5xx as success for exit status (post-script still runs first).
        #[arg(long)]
        allow_error_status: bool,
        /// The path pointing to the file to run
        #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        path: PathBuf,
    },
    /// Run multiple request files or multiple sequence files (see `--sequence`).
    Runall {
        /// Treat every `FILE` as a sequence definition (conflicts with `--request`).
        #[arg(long, short = 's', conflicts_with = "request")]
        sequence: bool,
        /// Every `FILE` is a single request (default; use to assert intent). Conflicts with `--sequence`.
        #[arg(long, conflicts_with = "sequence")]
        request: bool,
        /// Do not run `post_script` from the request or sequence file.
        #[arg(long)]
        no_post: bool,
        /// Expand and print only; no HTTP.
        #[arg(long)]
        dry_run: bool,
        /// Treat HTTP 4xx/5xx as success for exit status (post-script still runs first when present).
        #[arg(long)]
        allow_error_status: bool,
        /// Build the runtime environment once and reuse it across all files (variables persist between runs).
        #[arg(long)]
        retain_runtime: bool,
        /// Stop at the first failed file instead of running the rest.
        #[arg(long)]
        quit_on_failure: bool,
        #[arg(
            required = true,
            value_name = "FILE",
            value_hint = clap::ValueHint::FilePath,
            num_args = 1..
        )]
        files: Vec<PathBuf>,
    },
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
                eprintln!("no request files found");
            } else {
                for p in paths {
                    println!("{}", p.display());
                }
            }
        }
        Some(Command::Run { sequence, path, .. }) => {
            let opts = cmd_run::run_opts(&cli);
            if *sequence {
                cmd_run::run_sequence(path, &cli, opts).await?;
            } else {
                cmd_run::run_one(path, &cli, opts).await?;
            }
        }
        Some(Command::Runall {
            sequence,
            retain_runtime,
            quit_on_failure,
            files,
            ..
        }) => {
            let opts = cmd_run::run_opts(&cli);
            cmd_runall::run_all(
                files,
                *sequence,
                *retain_runtime,
                *quit_on_failure,
                &cli,
                opts,
            )
            .await?;
        }
        None => {
            let path = cli
                .file
                .as_ref()
                .ok_or_else(|| "expected a subcommand or a request file path".to_string())?;
            cmd_run::run_one(path, &cli, cmd_run::run_opts(&cli)).await?;
        }
    }
    return Ok(());
}

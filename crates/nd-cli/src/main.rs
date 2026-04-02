//! CLI entry for **nativedoctor**: `run`, `runall`, shorthand file path, `list`, `new`, and shared flags.

mod cmd_generate;
mod cmd_new;
mod cmd_rhai_definitions;
mod cmd_run;
mod cmd_web;
mod logging;
mod print;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{ArgAction, Parser, Subcommand, ValueEnum};

use crate::{
    cmd_new::NewOption, cmd_rhai_definitions::RhaiDefinitionsOptions, cmd_run::RunOptions,
};

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "File-based API request runner (JSON/YAML) with optional Rhai post-scripts.")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(author = "rubbie kelvin <dev.rubbie@gmail.com>")]
#[command(args_conflicts_with_subcommands = false)]
pub(crate) struct Cli {
    /// Load `KEY=value` pairs from each dotenv-style file into the runtime (later files override earlier).
    #[arg(long, value_name = "FILE", global = true, action = ArgAction::Append)]
    env: Vec<PathBuf>,

    /// Log extra detail and enable `nd_core=debug` tracing unless `RUST_LOG` is set.
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Persistence file, .yaml file where persisted values should be stored
    #[arg(long, value_name = "FILE", global = true)]
    persistence_file: Option<PathBuf>,

    /// Expand and print the request only; no network I/O (no request is actually run).
    #[arg(long, global = true)]
    no_network_io: bool,

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
    /// Run one or more request-file or rhai scripts
    Run {
        /// Build the runtime environment once and reuse it across all files (runtime variables persist between runs).
        #[arg(long)]
        retain_runtime: bool,
        /// The path pointing to the file(s) to run
        #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath, num_args = 1..)]
        paths: Vec<PathBuf>,
    },
    /// Serve web UI exposing directories directory.
    Web {
        /// Address and port to bind (default: loopback only).
        #[arg(long, value_name = "ADDR", default_value = "127.0.0.1:8080")]
        bind: SocketAddr,
        /// get the Directories whose top-level `*.json` / `*.yaml` / `*.yml` / `.rhai` files are listed.
        #[arg(long, value_name = "DIR", default_value = ".", value_hint = clap::ValueHint::FilePath, num_args = 1..)]
        dir: Vec<PathBuf>,
    },
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
    /// Write Rhai definition files (`.d.rhai`) for IDE / language-server support (builtins + nativedoctor globals).
    Definitions {
        /// Output directory for multiple definition files (see Rhai book: Engine definitions).
        #[arg(long, value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
        out_dir: Option<PathBuf>,
        /// Write a single merged definitions file instead of a directory.
        #[arg(long, value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
        out_file: Option<PathBuf>,
    },
    /// Create a starter request file
    New {
        /// Request url
        #[arg(long, short = 'u', value_name = "URL")]
        url: Option<String>,
        /// Request name
        #[arg(long, short = 'n', value_name = "NAME")]
        name: Option<String>,
        /// File name `*.json` / `*.yaml` / `*.yml` / files
        #[arg(value_name = "PATH")]
        path: PathBuf,
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
        Some(Command::Definitions { out_dir, out_file }) => {
            cmd_rhai_definitions::run_rhai_definitions(RhaiDefinitionsOptions {
                out_dir: out_dir.clone(),
                out_file: out_file.clone(),
            })?;
        }
        Some(Command::New { .. }) => {
            let opt = NewOption::from_cli(&cli);
            cmd_new::run_new(opt)?;
        }
        Some(Command::Run { .. }) => {
            let opts = RunOptions::from_cli(&cli)?;
            cmd_run::run_run(opts).await?;
        }
        #[allow(unused)]
        Some(Command::Web { bind, dir }) => {
            todo!("Not done yet");
            // let bind = *bind;
            // let dir = dir.clone();
            // let no_default_system_env = cli.no_default_system_env;
            // let env_files = cli.env.clone();
            // let verbose = cli.verbose;
            // tokio::task::spawn_blocking(move || {
            //     cmd_web::run(bind, dir, no_default_system_env, env_files, verbose)
            // })
            // .await
            // .map_err(|e| format!("web server task: {e}"))??;
        }
        None => {
            let opts = RunOptions::from_cli(&cli)?;
            cmd_run::run_run(opts).await?;
        }
    }
    return Ok(());
}

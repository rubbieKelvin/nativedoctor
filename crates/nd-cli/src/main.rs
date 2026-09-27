//! CLI entry: headless runner for local use and CI, backed by the same `nd-engine` the desktop app
//! uses ("one engine everywhere").

mod logging;

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};

use clap::{Parser, Subcommand, ValueEnum};

use nd_core::model::{HttpMethod, RequestRef};
use nd_core::Workspace;
use nd_engine::execute::types::PrintOptions;
use nd_engine::stream::Session;

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "Local-first API workspace runner (OpenAPI + Rhai)")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub(crate) struct Cli {
    /// Log extra detail and enable debug tracing unless `RUST_LOG` is set.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum MethodArg {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
}

impl From<MethodArg> for HttpMethod {
    fn from(m: MethodArg) -> Self {
        match m {
            MethodArg::Get => HttpMethod::Get,
            MethodArg::Put => HttpMethod::Put,
            MethodArg::Post => HttpMethod::Post,
            MethodArg::Delete => HttpMethod::Delete,
            MethodArg::Options => HttpMethod::Options,
            MethodArg::Head => HttpMethod::Head,
            MethodArg::Patch => HttpMethod::Patch,
            MethodArg::Trace => HttpMethod::Trace,
        }
    }
}

#[derive(Subcommand)]
enum Command {
    /// Create a new workspace (manifest + a starter OpenAPI document + `.env`).
    New {
        /// Directory the workspace folder is created under.
        #[arg(value_hint = clap::ValueHint::DirPath)]
        root: PathBuf,
        /// Workspace (and folder) name.
        name: String,
    },
    /// List every operation in a workspace's sidebar tree.
    List {
        #[arg(value_hint = clap::ValueHint::DirPath, default_value = ".")]
        workspace: PathBuf,
    },
    /// Send one request (an operation already in the workspace) and print the response.
    Run {
        #[arg(value_hint = clap::ValueHint::DirPath, default_value = ".")]
        workspace: PathBuf,
        /// OpenAPI path template, e.g. `/users/{id}`.
        #[arg(short, long)]
        path: String,
        #[arg(short, long, value_enum, default_value = "get")]
        method: MethodArg,
        /// Extra `KEY=value` env files, merged after the workspace's own (later files win).
        #[arg(long, value_name = "FILE")]
        env: Vec<PathBuf>,
        /// Stream the response body instead of buffering it.
        #[arg(long)]
        stream: bool,
    },
    /// Write Rhai definition files (`.d.rhai`) for IDE / language-server support.
    Definitions {
        #[arg(long, value_name = "DIR")]
        out_dir: Option<PathBuf>,
        #[arg(long, value_name = "FILE")]
        out_file: Option<PathBuf>,
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

async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::New { root, name } => {
            let path = nd_core::model::Manifest::create_in_path(&root, &name)
                .map_err(|e| e.to_string())?;
            println!("created workspace at {}", path.display());
        }
        Command::List { workspace } => {
            let ws = Workspace::open(&workspace).map_err(|e| e.to_string())?;
            for doc in &ws.manifest.openapi_docs {
                let Some(api) = ws.doc(doc) else { continue };
                for (path, method, op) in api.list_operations() {
                    let summary = op.summary.as_deref().unwrap_or("");
                    println!("{:<7} {:<30} {} {summary}", method.as_str(), path, doc.display());
                }
            }
        }
        Command::Run {
            workspace,
            path,
            method,
            env,
            stream,
        } => {
            let ws = Workspace::open(&workspace).map_err(|e| e.to_string())?;
            let runtime = ws.load_env().map_err(|e| e.to_string())?;
            for extra in &env {
                runtime.merge_env_file(extra).map_err(|e| e.to_string())?;
            }

            let session = Arc::new(Mutex::new(
                Session::new(|| Ok(runtime), None).map_err(|e| e)?,
            ));

            let request = RequestRef {
                doc: ws
                    .manifest
                    .openapi_docs
                    .first()
                    .cloned()
                    .ok_or_else(|| "workspace has no openapi documents".to_string())?,
                path,
                method: method.into(),
            };

            let result = nd_engine::run_request(&ws, &request, session, None, stream)
                .await
                .map_err(|e| e.to_string())?;

            result.print(PrintOptions::Verbose);

            if result.status == 0 || result.status >= 400 {
                return Err(format!("request completed with status {}", result.status));
            }
        }
        Command::Definitions { out_dir, out_file } => {
            if let Some(dir) = out_dir {
                nd_engine::rhai::definition_export::write_rhai_definition_files(&dir)
                    .map_err(|e| e.to_string())?;
            }
            if let Some(file) = out_file {
                nd_engine::rhai::definition_export::write_rhai_definitions_file(&file)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

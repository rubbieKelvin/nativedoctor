//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: request files or Rhai scripts.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use nd_core::execute::types::PrintOptions;
use nd_core::stream::events::Event;
use nd_core::stream::Session;
use nd_core::{
    env::RuntimeEnv,
    execute::format::format_prepared_request,
    model::request::RequestFile,
    rhai::{run_rhai_script, Logger, RhaiScriptRunOptions},
};

use crate::{Cli, Command};

#[derive(Debug, Clone)]
pub struct RunOptions {
    // Log extra detail
    pub verbose: bool,
    /// If true, returns immediately without I/O using a synthetic [`ExecutionResult`] (status 0).
    pub no_network_io: bool,
    /// Retain run time across runs
    pub retain_runtime: bool,
    /// requests/scripts to run
    pub paths: Vec<PathBuf>,
    pub persistence_file: Option<PathBuf>,
    pub env_files: Vec<PathBuf>,
}

impl RunOptions {
    pub(crate) fn from_cli(cli: &Cli) -> Result<RunOptions, String> {
        return Ok(match &cli.command {
            Some(Command::Run {
                retain_runtime,
                paths,
            }) => RunOptions {
                verbose: cli.verbose,
                no_network_io: cli.no_network_io,
                retain_runtime: *retain_runtime,
                paths: paths.clone(),
                persistence_file: cli.persistence_file.clone(),
                env_files: cli.env.clone(),
            },
            None => {
                let path = cli
                    .file
                    .clone()
                    .ok_or_else(|| "expected a subcommand or a request file path".to_string())?;

                RunOptions {
                    verbose: cli.verbose,
                    no_network_io: cli.no_network_io,
                    retain_runtime: true,
                    paths: vec![path],
                    persistence_file: cli.persistence_file.clone(),
                    env_files: cli.env.clone(),
                }
            }
            _ => unreachable!("Shouldn't get here"),
        });
    }
}

pub(crate) async fn run_run(opts: RunOptions) -> Result<(), String> {
    // create runtime session
    let mut session = Session::new(
        || {
            RuntimeEnv::new()
                .with_env_files(&opts.env_files)
                .map_err(|e| e.to_string())?
                .with_persistence(&opts.persistence_file)
                .map_err(|e| e.to_string())
        },
        None,
    )?;

    let logger = Arc::new(Logger::new());

    for path in opts.paths.iter() {
        let mut session = &mut session;

        if !path.try_exists().map_err(|e| e.to_string())? {
            tracing::error!(path = %path.display(), "File does not exist");
            session.emit(|e| Event::Error {
                elapsed: e,
                message: format!("File does not exist: {}", path.display()),
            });

            continue;
        }

        session.emit(|e| Event::FileLoaded {
            elapsed: e,
            path: path.clone(),
        });

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            "json" | "yaml" | "yml" => run_request(path, &opts, &mut session).await?,
            "rhai" => run_script(path, &opts, &mut session, logger.clone()).await?,
            _ => {
                return Err(String::from(
                    "Invalid file type. only json, yaml, yml, rhai files accepted",
                ))
            }
        };

        if !opts.retain_runtime {
            session.reload_runtime();
        }
    }

    return Ok(());
}

/// Run one request
pub async fn run_request(
    path: &Path,
    opts: &RunOptions,
    session: &mut Session,
) -> Result<(), String> {
    let document = RequestFile::from_file(path).map_err(|e| e.to_string())?;

    // return run_one_with_env(path, cli, opts, &env).await;
    if opts.verbose && !opts.no_network_io {
        println!("{}", format!("--- request/{:?} ---", document.name));
    }

    if opts.no_network_io || opts.verbose {
        let request = document
            .request
            .expand(&session.runtime)
            .map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&request).map_err(|e| e.to_string())?;
        println!("{summary}");

        if opts.no_network_io {
            return Ok(());
        }
    }

    if opts.verbose {
        println!("--- response/{:?} ---", document.name);
    }

    let output = document
        .execute(&session.runtime)
        .await
        .map_err(|e| e.to_string())?;

    output.print(if opts.verbose {
        PrintOptions::Verbose
    } else {
        PrintOptions::Normal
    });

    return Ok(());
}

pub async fn run_script(
    path: &Path,
    opts: &RunOptions,
    session: &mut Session,
    logger: Arc<Logger>,
) -> Result<(), String> {
    if opts.verbose {
        println!("--- script/{} ---", path.display());
    }

    if opts.no_network_io {
        tracing::warn!(
            path = %path.display(),
            "--no-network-io is ignored for scripts"
        );
    }

    return run_rhai_script(
        path,
        &session.runtime,
        Some(logger),
        RhaiScriptRunOptions {
            no_network_io: opts.no_network_io,
        },
    )
    .map_err(|e| e.to_string());
}

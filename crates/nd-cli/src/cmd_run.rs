//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: single request, or `run --sequence <FILE>` for sequences.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use nd_core::{
    env::RuntimeEnv,
    execute::format::format_prepared_request,
    model::request::RequestFile,
    rhai::{run_rhai_script, Logger},
};

use crate::{print::print_result, Cli, Command};

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
    let runtime = RuntimeEnv::new()
        .with_env_files(&opts.env_files)
        .map_err(|e| e.to_string())?
        .with_persistence(&opts.persistence_file)
        .map_err(|e| e.to_string())?;
    let logger = Arc::new(Logger::new());

    for path in opts.paths.iter() {
        if !path.try_exists().map_err(|e| e.to_string())? {
            tracing::error!(path = %path.display(), "File does not exist");
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            "json" | "yaml" | "yml" => run_request(path, &opts, &runtime).await?,
            "rhai" => run_script(path, &opts, &runtime, logger.clone()).await?,
            _ => {
                return Err(String::from(
                    "Invalid file type. only json, yaml, yml, rhai files accepted",
                ))
            }
        };

        if !opts.retain_runtime {
            runtime.clear();
        }
    }

    return Ok(());
}

/// Run one request
pub async fn run_request(path: &Path, opts: &RunOptions, env: &RuntimeEnv) -> Result<(), String> {
    let document = RequestFile::from_file(path).map_err(|e| e.to_string())?;

    // return run_one_with_env(path, cli, opts, &env).await;
    if opts.verbose && !opts.no_network_io {
        println!("{}", format!("--- request/{:?} ---", document.name));
    }

    if opts.no_network_io || opts.verbose {
        let request = document.request.expand(env).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&request).map_err(|e| e.to_string())?;
        println!("{summary}");

        if opts.no_network_io {
            return Ok(());
        }
    }

    if opts.verbose {
        println!("--- response/{:?} ---", document.name);
    }

    let output = document.execute(&env).await.map_err(|e| e.to_string())?;
    print_result(&output, opts.verbose)?;

    return Ok(());
}

pub async fn run_script(
    path: &Path,
    opts: &RunOptions,
    env: &RuntimeEnv,
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

    return run_rhai_script(path, env, Some(logger)).map_err(|e| e.to_string());
}

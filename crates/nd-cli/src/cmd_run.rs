//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: single request, or `run --sequence <FILE>` for sequences.

use std::path::{Path, PathBuf};

use colored::Colorize;
use nd_core::{
    env::RuntimeEnv, execute::format::format_prepared_request, model::request::RequestFile,
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
    pub(crate) fn from_cli(cli: &Cli) -> RunOptions {
        return match &cli.command {
            Some(Command::Run {
                no_network_io,
                retain_runtime,
                paths,
                persistence_file,
            }) => RunOptions {
                verbose: cli.verbose,
                no_network_io: *no_network_io,
                retain_runtime: *retain_runtime,
                paths: paths.clone(),
                persistence_file: *persistence_file,
                env_files: cli.env.clone(),
            },
            _ => unreachable!("Shouldn't get here"),
        };
    }
}

pub(crate) async fn run_run(opts: RunOptions) -> Result<()> {
    // create runtime session
    let runtime = RuntimeEnv::new()
        .with_env_files(opts.env_files)?
        .with_persistence(opts.persistence_file)?;

    for path in opts.paths.iter() {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            "json" | "yaml" | "yml" => run_request(path, &opts, env).await?,
            "rhai" => run_script(path, &opts, env).await?,
        };

        run_request(path, &opts, env).await?;

        if !opts.retain_runtime {
            runtime.clear();
        }
    }
}

/// Run one request
pub async fn run_request(path: &Path, opts: &RunOptions, env: &RuntimeEnv) -> Result<(), String> {
    let document = RequestFile::from_file(path)?;

    // return run_one_with_env(path, cli, opts, &env).await;
    if opts.verbose && !opts.no_network_io {
        println!(format!("--- request/{:?} ---", document.name));
    }

    if opts.no_network_io || opts.verbose {
        let request = document.request.expand(env)?;
        let summary = format_prepared_request(&request).map_err(|e| e.to_string())?;
        println!("{summary}");

        if opts.no_network_io {
            return Ok(());
        }
    }

    if opts.verbose {
        println!("--- response/{:?} ---", document.name);
    }

    let output = document.execute(&env).await?.map_err(|e| e.to_string())?;
    print_result(&output, cli.verbose)?;

    return Ok(());
}

pub async fn run_script(path: &Path, opts: &RunOptions, env: &RuntimeEnv) -> Result<(), String> {
    todo!("I havent imlpemented this fearture yet")
}

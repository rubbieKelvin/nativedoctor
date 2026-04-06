//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: request files or Rhai scripts.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use nd_core::execute::types::PrintOptions;
use nd_core::rhai::logger::log_parsed_level;
use nd_core::stream::events::Event;
use nd_core::stream::{MutexSession, Session};
use nd_core::{
    env::RuntimeEnv,
    execute::format::format_prepared_request,
    model::request::RequestFile,
    rhai::{resolver::RhaiScriptRunOptions, run::run_rhai_script},
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
    pub stream: bool,
}

impl RunOptions {
    pub(crate) fn from_cli(cli: &Cli) -> Result<RunOptions, String> {
        return Ok(match &cli.command {
            Some(Command::Run {
                retain_runtime,
                stream_content,
                paths,
            }) => RunOptions {
                stream: stream_content.clone(),
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
                    stream: false,
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

fn handle_session_events_for_cli(event: Event) {
    match event {
        Event::Log {
            level,
            message,
            script,
            ..
        } => {
            log_parsed_level(level.as_str(), message, script);
        }
        _ => {}
    };
}

pub(crate) async fn run_run(opts: RunOptions) -> Result<(), String> {
    // create runtime session
    let session = Arc::new(Mutex::new(Session::new(
        || {
            RuntimeEnv::new()
                .with_env_files(&opts.env_files)
                .map_err(|e| e.to_string())?
                .with_persistence(&opts.persistence_file)
                .map_err(|e| e.to_string())
        },
        Some(Box::new(|event: Event| {
            handle_session_events_for_cli(event);
        })),
    )?));

    for path in opts.paths.iter() {
        let session = session.clone();

        if !path.try_exists().map_err(|e| e.to_string())? {
            tracing::error!(path = %path.display(), "File does not exist");
            session.emit(|id, e| Event::Error {
                session_id: id,
                elapsed: e,
                message: format!("File does not exist: {}", path.display()),
            });

            continue;
        }

        session.emit(|id, e| Event::FileLoaded {
            session_id: id,
            elapsed: e,
            path: path.clone(),
        });

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            "json" | "yaml" | "yml" => run_request(path, &opts, session.clone()).await?,
            "rhai" => run_script(path, &opts, session.clone()).await?,
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
    session: Arc<Mutex<Session>>,
) -> Result<(), String> {
    let document = RequestFile::from_file(path).map_err(|e| e.to_string())?;

    // return run_one_with_env(path, cli, opts, &env).await;
    if opts.verbose && !opts.no_network_io {
        println!("{}", format!("--- request/{:?} ---", document.name));
    }

    if opts.no_network_io || opts.verbose {
        let request = document
            .request
            .expand(&session.runtime())
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
        .execute(session, opts.stream.clone())
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
    session: Arc<Mutex<Session>>,
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
        session,
        RhaiScriptRunOptions {
            no_network_io: opts.no_network_io,
        },
    )
    .map_err(|e| e.to_string());
}

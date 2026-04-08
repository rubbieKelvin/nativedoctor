//! Async run loop and [`TuiMsg`] bridge to the UI task.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use nd_core::env::RuntimeEnv;
use nd_core::execute::format::format_prepared_request;
use nd_core::model::request::RequestFile;
use nd_core::rhai::{resolver::RhaiScriptRunOptions, run::run_rhai_script};
use nd_core::stream::events::Event;
use nd_core::stream::{MutexSession, Session};
use tokio::sync::mpsc::UnboundedSender;

use crate::ui;

/// Options for a TUI run (mirrors `nativedoctor run` / global CLI flags).
#[derive(Clone, Debug)]
pub struct TuiOptions {
    pub verbose: bool,
    pub no_network_io: bool,
    pub retain_runtime: bool,
    pub paths: Vec<PathBuf>,
    pub persistence_file: Option<PathBuf>,
    pub env_files: Vec<PathBuf>,
    /// When true, HTTP body is streamed via session events (`HttpResponseStream*`).
    pub stream: bool,
}

/// Messages from the runner and session into the TUI state machine.
#[derive(Debug)]
pub enum TuiMsg {
    Event(Event),
    RequestSummary(String),
    BufferedHttpBody {
        status: u16,
        final_url: String,
        body: Vec<u8>,
    },
    RunnerFinished(Result<(), String>),
}

pub async fn run_tui(opts: TuiOptions) -> Result<(), String> {
    if opts.paths.is_empty() {
        return Err("expected at least one request file or Rhai script path".into());
    }

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<TuiMsg>();
    let tx_event = tx.clone();

    let session = Arc::new(Mutex::new(Session::new(
        || {
            RuntimeEnv::new()
                .with_env_files(&opts.env_files)
                .map_err(|e| e.to_string())?
                .with_persistence(&opts.persistence_file)
                .map_err(|e| e.to_string())
        },
        Some(Box::new(move |ev: Event| {
            let _ = tx_event.send(TuiMsg::Event(ev));
        })),
    )?));

    let run_opts = opts.clone();
    let sess = session.clone();
    let tx_done = tx.clone();
    tokio::spawn(async move {
        let result = run_paths_loop(&run_opts, sess, &tx_done).await;
        let _ = tx_done.send(TuiMsg::RunnerFinished(result));
    });

    ui::run_terminal(rx).await
}

async fn run_paths_loop(
    opts: &TuiOptions,
    session: Arc<Mutex<Session>>,
    tx: &UnboundedSender<TuiMsg>,
) -> Result<(), String> {
    for path in opts.paths.iter() {
        let session = session.clone();

        if !path.try_exists().map_err(|e| e.to_string())? {
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
            "json" | "yaml" | "yml" => {
                run_request_tui(path, opts, session.clone(), tx).await?;
            }
            "rhai" => {
                run_script_tui(path, opts, session.clone()).await?;
            }
            _ => {
                return Err(String::from(
                    "Invalid file type. only json, yaml, yml, rhai files accepted",
                ));
            }
        }

        if !opts.retain_runtime {
            session.reload_runtime();
        }
    }

    Ok(())
}

async fn run_request_tui(
    path: &Path,
    opts: &TuiOptions,
    session: Arc<Mutex<Session>>,
    tx: &UnboundedSender<TuiMsg>,
) -> Result<(), String> {
    let document = RequestFile::from_file(path).map_err(|e| e.to_string())?;

    if opts.no_network_io || opts.verbose {
        let request = document
            .request
            .expand(&session.runtime())
            .map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&request).map_err(|e| e.to_string())?;
        let _ = tx.send(TuiMsg::RequestSummary(summary));
        if opts.no_network_io {
            return Ok(());
        }
    }

    let output = document
        .execute(session, opts.stream)
        .await
        .map_err(|e| e.to_string())?;

    if !opts.stream {
        let _ = tx.send(TuiMsg::BufferedHttpBody {
            status: output.status,
            final_url: output.final_url,
            body: output.body,
        });
    }

    Ok(())
}

async fn run_script_tui(
    path: &Path,
    opts: &TuiOptions,
    session: Arc<Mutex<Session>>,
) -> Result<(), String> {
    run_rhai_script(
        path,
        session,
        RhaiScriptRunOptions {
            no_network_io: opts.no_network_io,
        },
    )
    .map_err(|e| e.to_string())
}

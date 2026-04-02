//! Local HTTP server for the NativeDoctor web UI: JSON API plus a Vue SPA shipped as embedded static assets.
//!
//! The SPA is built by this crate's `build.rs` (`pnpm build` → `frontend/dist`) and embedded with the `rust-embed` crate.
//! Run the binary via **`nativedoctor web`** from `nd-cli`, or depend on this crate from other tools.

mod api;
pub mod embed;
mod path_sandbox;
mod workspace;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use nd_core::env::RuntimeEnv;
use tracing::info;

pub use api::{app_router, AppState};

/// Configuration for [`run_web`], mirroring global `nativedoctor` CLI flags for the `web` subcommand.
pub struct WebServerOptions {
    /// Socket address for the HTTP listener (e.g. `127.0.0.1:8080`).
    pub bind: SocketAddr,
    /// Workspace roots; each directory is scanned non-recursively for request and Rhai files.
    pub roots: Vec<PathBuf>,
    /// Dotenv-style files merged into [`nd_core::env::RuntimeEnv`] (later files override).
    pub env_files: Vec<PathBuf>,
    /// Optional persistence file for Rhai/runtime `persist` (see `nd-core`).
    pub persistence_file: Option<PathBuf>,
    /// When `true`, HTTP requests are expanded only (no outbound I/O); Rhai still runs but honors script options.
    pub no_network_io: bool,
}

/// Binds [`opts.bind`](WebServerOptions::bind), installs [`app_router`], and serves until the process is interrupted.
pub async fn run_web(opts: WebServerOptions) -> anyhow::Result<()> {
    let roots_vec = path_sandbox::canonicalize_roots(&opts.roots).map_err(anyhow::Error::msg)?;
    let roots = Arc::new(roots_vec);

    let env = RuntimeEnv::new()
        .with_env_files(&opts.env_files)
        .map_err(anyhow::Error::from)?
        .with_persistence(&opts.persistence_file)
        .map_err(anyhow::Error::from)?;

    let state = AppState {
        roots: roots.clone(),
        env: Arc::new(env),
        no_network_io: opts.no_network_io,
    };

    let app: Router = api::app_router(state);

    let listener = tokio::net::TcpListener::bind(opts.bind).await?;
    info!(addr = %opts.bind, roots = ?roots.as_ref(), "nd-web listening");

    axum::serve(listener, app).await?;

    Ok(())
}

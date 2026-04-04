//! Local HTTP server for the NativeDoctor web UI: JSON API plus a Vue SPA.
//!
//! **Release** builds embed `frontend/dist` via `rust-embed`. **Debug** builds serve the same files from
//! disk so development binaries do not embed asset contents. In **debug**, [`run_web`]
//! also starts **`pnpm run dev`** (Vite on port 5173 with HMR) unless `ND_WEB_SKIP_VITE_DEV=1`; stop
//! the Rust server (Ctrl+C) to tear down Vite as well. The crate `build.rs` runs `pnpm build` only for
//! release.
//!
//! Run the binary via **`nativedoctor web`** from `nd-cli` or use this crate directely.

mod api;
pub mod embed;
mod path_sandbox;
mod workspace;

#[cfg(debug_assertions)]
mod vite;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use nd_core::env::RuntimeEnv;
use tracing::info;

pub use api::{api_router, app_router, AppState};

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

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };

    #[cfg(unix)]
    {
        let mut sigterm =
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(s) => s,
                Err(_) => {
                    ctrl_c.await;
                    return;
                }
            };

        tokio::select! {
            () = ctrl_c => {},
            _ = sigterm.recv() => {},
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await;
    }
}

/// Binds [`opts.bind`](WebServerOptions::bind), installs [`app_router`], and serves until the process is interrupted.
pub async fn run_web(opts: WebServerOptions) -> anyhow::Result<()> {
    let roots_vec = path_sandbox::canonicalize_roots(&opts.roots).map_err(anyhow::Error::msg)?;
    let roots = Arc::new(roots_vec);

    // TODO: runtime env is run specific, so we should only have this when we're running a request or script
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

    // vite server proccess will shut down when we drop this variable
    #[cfg(debug_assertions)]
    let _vite_dev = match vite::maybe_start_vite_dev(&opts.bind)? {
        Some(guard) => {
            info!(
                vite = "http://127.0.0.1:5173",
                api = %opts.bind,
                "Vite dev server (HMR); open the vite URL. JSON API is proxied from Vite to this bind address."
            );
            Some(guard)
        }
        None => {
            info!("Vite dev not started (set ND_WEB_SKIP_VITE_DEV=1). Use frontend/dist on the API bind or run `pnpm dev` in crates/nd-web/frontend.");
            None
        }
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

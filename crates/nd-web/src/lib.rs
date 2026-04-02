//! Local HTTP server for the NativeDoctor web UI (API + static SPA).

mod api;
mod path_sandbox;
mod workspace;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use nd_core::env::RuntimeEnv;
use tracing::info;

pub use api::AppState;
pub use workspace::dist_dir;

/// Options aligned with `nativedoctor web` CLI flags.
pub struct WebServerOptions {
    pub bind: SocketAddr,
    pub roots: Vec<PathBuf>,
    pub env_files: Vec<PathBuf>,
    pub persistence_file: Option<PathBuf>,
    pub no_network_io: bool,
}

/// Start the Axum server (blocks until shutdown).
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

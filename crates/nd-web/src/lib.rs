//! Local web UI for nativedoctor: list and run request files and sequences (Dioxus 0.7 fullstack, server).

mod app;
mod bootstrap;
mod config;
mod path_utils;
mod server;

pub mod components;
pub mod views;

use std::net::SocketAddr;
use std::path::PathBuf;

pub use app::App;

use crate::config::WebConfig;

/// Configure workspace, bind address, and runtime options; then start the Dioxus Axum server.
/// Blocks while the server runs.
pub fn run_web(
    root: PathBuf,
    bind: SocketAddr,
    no_default_system_env: bool,
    env_files: Vec<PathBuf>,
    verbose: bool,
) -> Result<(), String> {
    config::set_web_config(WebConfig {
        root,
        no_default_system_env,
        env_files,
        verbose,
    })?;

    bootstrap::ensure_public_dir()?;
    bootstrap::set_server_listen_addr(bind);

    dioxus::LaunchBuilder::server().launch(crate::app::App);

    #[allow(unreachable_code)]
    return Ok(());
}

//! `nativedoctor web` — local API + Vue UI for listing and running request files and Rhai scripts.

use std::net::SocketAddr;
use std::path::PathBuf;

pub async fn run(
    bind: SocketAddr,
    dirs: Vec<PathBuf>,
    env_files: Vec<PathBuf>,
    persistence_file: Option<PathBuf>,
    no_network_io: bool,
) -> Result<(), String> {
    println!(
        "Web UI: http://{} (roots: {})",
        bind,
        dirs.iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    nd_web::run_web(nd_web::WebServerOptions {
        bind,
        roots: dirs,
        env_files,
        persistence_file,
        no_network_io,
    })
    .await
    .map_err(|e| e.to_string())
}

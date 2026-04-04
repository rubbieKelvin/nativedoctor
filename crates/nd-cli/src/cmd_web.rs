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
    let roots_list = dirs
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("API: http://{bind} (roots: {roots_list})");

    #[cfg(debug_assertions)]
    println!(
        "Debug UI (Vite HMR): http://127.0.0.1:5173 — proxied to this API. Set ND_WEB_SKIP_VITE_DEV=1 to skip auto `pnpm dev`."
    );

    #[cfg(not(debug_assertions))]
    println!("Web UI is served from the API URL above.");

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

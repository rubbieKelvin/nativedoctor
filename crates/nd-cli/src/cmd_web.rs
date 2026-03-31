//! `nativedoctor web` — local Dioxus UI for listing and running request files.

use std::net::SocketAddr;
use std::path::PathBuf;

pub fn run(
    bind: SocketAddr,
    dir: PathBuf,
    no_default_system_env: bool,
    env_files: Vec<PathBuf>,
    verbose: bool,
) -> Result<(), String> {
    println!(
        "Running web UI on http://{} (dir: {})",
        bind,
        dir.display()
    );
    nd_web::run_web(dir, bind, no_default_system_env, env_files, verbose)
}

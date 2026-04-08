//! `nativedoctor tui` — interactive ratatui runner for request files and Rhai scripts.

use crate::Cli;

pub async fn run(
    cli: &Cli,
    paths: Vec<std::path::PathBuf>,
    retain_runtime: bool,
    stream: bool,
) -> Result<(), String> {
    nd_tui::run_tui(nd_tui::TuiOptions {
        verbose: cli.verbose,
        no_network_io: cli.no_network_io,
        retain_runtime,
        paths,
        persistence_file: cli.persistence_file.clone(),
        env_files: cli.env.clone(),
        stream,
    })
    .await
}

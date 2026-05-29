mod app;
mod cli;
mod theme;
mod ui;
mod windows;

use std::path::PathBuf;

use clap::Parser;
use nd_core::model::project::ProjectFile;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nd_app=info,warn".into()),
        )
        .init();

    let cli = cli::Cli::parse();

    match cli.command {
        Some(cli::Command::Init { name, path }) => {
            let root = path
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

            match ProjectFile::create_in_path(root, name) {
                Ok(()) => tracing::info!("Project created successfully"),
                Err(e) => {
                    tracing::error!("Failed to create project: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None => {
            tracing::info!("Starting NativeDoctor desktop application");
            app::setup();
        }
    }
}

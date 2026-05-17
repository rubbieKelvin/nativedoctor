mod app;
mod cli;
mod files;
mod ui;

use std::path::PathBuf;

use clap::Parser;

fn main() {
    // Initialise tracing for debug logs.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nd_app=info,warn".into()),
        )
        .init();

    let cli = cli::Cli::parse();

    match cli.command {
        // create a project with "init"
        Some(cli::Command::Init { name, path }) => {
            let root = path.unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            });

            match files::create_project(root, name) {
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

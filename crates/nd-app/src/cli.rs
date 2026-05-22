use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nativedoctor", about = "NativeDoctor API client")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Initialize a new project.
    Init {
        /// Project name.
        name: String,
        /// Root directory where the project folder will be created (defaults to current directory).
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

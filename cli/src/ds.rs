use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub enum RunMode {
    Request(String),
    // Sequence(String),
    // All,
}

#[derive(Parser, Debug)]
#[command(
    name = "rustle",
    about = "a friendly and robust API testing tool crafted in Rust",
    version = "0.0.1"
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init {
        name: String,
    },
    /// Runs from a config file
    Run {
        /// The filepath to the .api.yaml file to load
        filepath: PathBuf,
        /// the enviroment to run on. defaults to none
        #[arg(short, long)]
        env: Option<String>,
        /// if specified the the request with the request name will be run.
        /// else all sequence will run (not to be use with sequence arg)
        #[arg(short, long)]
        request: Option<String>,
        /// If specified, will run all request in the specified sequence.
        /// else all sequence will run (not to be use with request arg)
        #[arg(short, long)]
        sequence: Option<String>,
    },
}

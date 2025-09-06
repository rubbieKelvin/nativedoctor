use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Native doctor")]
#[command(
    about = "API testing tool designed for easy YAML definition and seamless integration into CI/CD pipelines."
)]
#[command(version, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Create new native doctor project
    New (NewArgs),
    /// Run a request or a call
    Run(RunArgs)
}

#[derive(Args)]
pub struct NewArgs {
    /// Create a single request file
    #[arg(short, long)]
    pub request: bool,
    pub name: Option<String>,
}


#[derive(Args)]
pub struct RunArgs {
    /// The path of the request file or the call file to run
    pub path: PathBuf,
    /// Do not call request dependencies
    #[arg(long = "no-deps")]
    pub no_deps: bool
}
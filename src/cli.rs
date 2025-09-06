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
}

#[derive(Args)]
pub struct NewArgs {
    /// Create a single request file
    #[arg(short, long)]
    pub request: bool,
    pub name: Option<String>,
}

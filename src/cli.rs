use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Create new native doctor project
    New {
        /// Create a single request file
        #[arg(short, long)]
        request: bool
    }
}
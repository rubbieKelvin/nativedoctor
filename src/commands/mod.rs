use clap::{Parser, Subcommand};

pub mod env;
pub mod init;

#[derive(Parser, Debug)]
#[command(
    name = "dotapi",
    about = "An api client testing tool",
    version = "0.0.1"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a dotapi project
    Init { path: Option<String> },
    /// Adds a request to the project
    Add {},
    /// Sets an enviroment variable for the project
    Env {
        /// Name of the enviroment. if not specified the default is used
        #[arg(short, long)]
        name: Option<String>,

        #[command(subcommand)]
        command: EnvSubcommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvSubcommands {
    Set {
        /// The key of the variable
        #[arg(short, long)]
        key: String,

        /// The value for the variable
        value: String,
    },
    UnSet {
        /// The key of the variable
        #[arg(short, long)]
        key: String,
    },
    Delete,
    List {
        /// The name of the enviroment to list. if not specified the default is used
        #[arg(short, long)]
        name: Option<String>,
    },
}

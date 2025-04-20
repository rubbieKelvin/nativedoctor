use clap::{Parser, Subcommand};
use commands::init;

mod commands;
mod schema;

#[derive(Parser, Debug)]
#[command(
    name = "dotapi",
    about = "An api client testing tool",
    version = "0.0.1"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init { path: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    // check sub command
    if let Some(command) = &cli.command {
        match command {
            Commands::Init { path } => {
                match init::init(path) {
                    Err(e) => eprintln!("{e}"),
                    _ => (),
                };
            }
        }
    } else {
        eprintln!("pass --help to see usage");
    }
}

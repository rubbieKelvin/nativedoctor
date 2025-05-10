use clap::Parser;
use ds::{Cli, Commands, RunMode};
use init::init;

mod ds;
mod init;
mod run;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(command) = &cli.commands {
        match command {
            Commands::Init { name } => {
                init(name);
            }
            Commands::Run {
                filepath,
                env,
                request,
                sequence,
            } => {
                run::run(
                    filepath,
                    env.clone(),
                    if request.is_some() {
                        RunMode::Request(request.clone().unwrap())
                    } else if sequence.is_some() {
                        RunMode::Sequence(sequence.clone().unwrap())
                    } else {
                        RunMode::All
                    },
                )
                .await;
            }
        };
    } else {
        eprintln!("pass --help to see usage");
    }
}

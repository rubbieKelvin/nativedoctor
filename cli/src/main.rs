use clap::Parser;
use ds::{Cli, Commands, RunMode};
use init::init;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod ds;
mod init;
mod run;

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Set the maximum logging level
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    let cli = Cli::parse();

    if let Some(command) = &cli.commands {
        match command {
            Commands::Init { name } => {
                init(name);
            }
            #[allow(unused)]
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

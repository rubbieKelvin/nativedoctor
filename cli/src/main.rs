use actions::{init::init, run::run, tree::draw_tree};
use clap::Parser;
use ds::{Cli, Commands, RunMode};
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

mod actions;
mod ds;

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::INFO) // Set the maximum logging level
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("Failed to set global default subscriber");

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
                run(
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
            Commands::Tree { filepath } => {
                draw_tree(filepath);
            }
        }
    } else {
        eprintln!("pass --help to see usage");
    }
}

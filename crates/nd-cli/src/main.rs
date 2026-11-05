//! `nativedoctor`: Postman for the command line.

mod args;
mod commands;
mod exchange;
mod failure;
mod render;

use std::process::ExitCode;

use clap::Parser;

use args::{Cli, Command};

#[tokio::main]
async fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let _ = error.print();
            // Clap's usual 2 is our timeout code.
            return ExitCode::from(u8::from(error.use_stderr()));
        }
    };

    let outcome = match cli.command {
        Command::Init { dir, name } => commands::init(&dir, name.as_deref()),
        Command::Run(args) => commands::run(args).await,
        Command::Send(args) => commands::send(args).await,
        Command::List { path } => commands::list(&path),
        Command::Check { path, env, vars } => commands::check(&path, env.as_deref(), &vars),
    };

    return match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            if !failure.message.is_empty() {
                eprintln!("error: {}", failure.message);
            }
            ExitCode::from(failure.code)
        }
    };
}

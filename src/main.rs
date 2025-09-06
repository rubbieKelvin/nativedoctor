use anyhow::Ok;
use clap::Parser;

use crate::{
    cli::{Cli, SubCommand},
    commands::new::create_request_file,
    utils::get_current_directory,
};

mod cli;
mod commands;
mod constants;
mod schemas;
#[cfg(test)]
mod tests;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let current_directory = get_current_directory()?;
    let commandline = Cli::parse();

    match &commandline.subcommand {
        Some(SubCommand::New(arg)) => {
            if arg.request {
                // create single request file in the current directory
                let name = arg.name.clone().unwrap_or(String::from("hello"));
                create_request_file(name, &current_directory)?;
            } else {
                // create project folder in the current directory
                todo!("Should implement this soon")
            }
        }
        _ => {
            // Maybe show the --help here somewhere
            eprintln!("Invalid command");
        }
    };

    return Ok(());
}

use anyhow::Ok;
use clap::Parser;

use crate::{
    cli::{Cli, SubCommand},
    commands::{
        new::{create_project_folder, create_request_file},
        run::run_native_doctor_path,
    },
    utils::get_current_directory,
};

mod cli;
mod commands;
mod constants;
mod defs;
mod schemas;
#[cfg(test)]
mod tests;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    let current_directory = get_current_directory()?;
    let commandline = Cli::parse();

    match &commandline.subcommand {
        // Handle new
        Some(SubCommand::New(arg)) => {
            let name = arg.name.clone().unwrap_or(String::from("request"));

            if arg.request {
                // create single request file in the current directory
                create_request_file(&name, &current_directory)?;
            } else {
                // create project folder in the current directory
                create_project_folder(&name, &current_directory)?;
            }
        }
        // Handle run command
        Some(SubCommand::Run(arg)) => {
            run_native_doctor_path(arg.input.clone(), arg.no_deps, &current_directory)?;
        }
        _ => {
            // Maybe show the --help here somewhere
            eprintln!("Invalid command");
        }
    };

    return Ok(());
}

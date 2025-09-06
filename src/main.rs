use std::fs::canonicalize;

use anyhow::{Context, Ok, bail};
use clap::Parser;

use crate::{
    cli::{Cli, SubCommand},
    commands::{new::{create_project_folder, create_request_file}, run::run_native_doctor_path},
    utils::get_current_directory,
};

mod cli;
mod commands;
mod constants;
mod schemas;
#[cfg(test)]
mod tests;
mod utils;
mod defs;

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
            let path = if arg.path.is_absolute() {
                // if it's absolute, we can use it as is
                &arg.path
            } else {
                let rough_path = &current_directory.join(&arg.path);
                &canonicalize(rough_path).context("Failed to canonicalize path")?
            };

            // path has to exist
            if !path.try_exists()? {
                bail!("No such file: {:?}", path)
            }

            // path has to be a file
            if !path.is_file() {
                bail!("Path is not a file")
            }

            run_native_doctor_path(path, arg.no_deps)?;
        }
        _ => {
            // Maybe show the --help here somewhere
            eprintln!("Invalid command");
        }
    };

    return Ok(());
}

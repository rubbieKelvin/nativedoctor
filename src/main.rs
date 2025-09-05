use clap::Parser;

use crate::cli::{Cli, SubCommand};

mod cli;
mod schemas;
mod commands;
mod utils;

fn main() {
    let commandline = Cli::parse();
    
    match &commandline.subcommand {
        Some(SubCommand::New { request }) => {
            if *request {
                // create single request file in the current directory
            }else{
                // create project folder in the current directory
                todo!("Should implement this soon")
            }
        },
        _ => {
            // Maybe show the --help here somewhere
            eprintln!("Invalid command");
        }
    };
}

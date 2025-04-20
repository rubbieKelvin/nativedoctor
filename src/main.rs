use clap::Parser;
use commands::{env, init, Cli, Commands, EnvSubcommands};
use constants::DEFAULT_ENVIROMENT_NAME;

mod commands;
mod constants;
mod schema;
mod utils;

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    // check sub command
    if let Some(command) = &cli.command {
        return match command {
            Commands::Init { path } => init::init(path),
            Commands::Add {} => {
                return Ok(());
            }
            Commands::Env { name, command } => {
                let abs_name = match name {
                    Some(name) => name.to_owned(),
                    None => DEFAULT_ENVIROMENT_NAME.to_owned(),
                };

                return match command {
                    EnvSubcommands::Set { key, value } => {
                        env::set_enviroment_variable(abs_name.as_str(), key, value)
                    }
                    EnvSubcommands::UnSet { key } => {
                        env::unset_enviroment_variable(abs_name.as_str(), key)
                    }
                    EnvSubcommands::Delete => env::delete_env_record(abs_name.as_str()),
                    EnvSubcommands::List { name } => env::list_env_records(name.clone()),
                };
            } // _ => Err("Unimplemented command".to_string()),
        };
    } else {
        // eprintln!("pass --help to see usage");
        return Err("pass --help to see usage".to_string());
    }
}

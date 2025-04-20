use clap::Parser;
use commands::{Cli, Commands, EnvSubcommands};
use constants::DEFAULT_ENVIROMENT_NAME;

mod commands;
mod constants;
mod schema;
mod utils;

#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    // check sub command
    if let Some(command) = &cli.command {
        return match command {
            Commands::Init { path } => commands::init::init(path),
            Commands::Add {
                name,
                url,
                method,
                params,
                headers,
                body,
                form,
                files,
                auth,
                bearer,
                api_key,
                api_key_header,
            } => commands::add::add(
                name.clone(),
                url.clone(),
                method.clone(),
                params.clone(),
                headers.clone(),
                body.clone(),
                form.clone(),
                files.clone(),
                auth.clone(),
                bearer.clone(),
                api_key.clone(),
                api_key_header.clone(),
            ),
            Commands::Env { name, command } => {
                let abs_name = match name {
                    Some(n) => n.to_owned(),
                    None => DEFAULT_ENVIROMENT_NAME.to_owned(),
                };

                return match command {
                    EnvSubcommands::Set { key, value } => {
                        commands::env::set_enviroment_variable(abs_name.as_str(), key, value)
                    }
                    EnvSubcommands::UnSet { key } => {
                        commands::env::unset_enviroment_variable(abs_name.as_str(), key)
                    }
                    EnvSubcommands::Delete => commands::env::delete_env_record(abs_name.as_str()),
                    EnvSubcommands::List => commands::env::list_env_records(name.clone()),
                };
            }
            Commands::Call { name, env } => commands::call::call(name, env.clone()).await,
            Commands::Ls => commands::ls::ls(),
        };
    } else {
        // eprintln!("pass --help to see usage");
        return Err("pass --help to see usage".to_string());
    }
}

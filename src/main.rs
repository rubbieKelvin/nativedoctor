use clap::Parser;
use commands::{env, init, Cli, Commands, EnvSubcommands};
use constants::DEFAULT_ENVIROMENT_NAME;
use utils::{get_current_project_config_path, load_config};

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
                        env::set_enviroment_variable(abs_name.as_str(), key, value)
                    }
                    EnvSubcommands::UnSet { key } => {
                        env::unset_enviroment_variable(abs_name.as_str(), key)
                    }
                    EnvSubcommands::Delete => env::delete_env_record(abs_name.as_str()),
                    EnvSubcommands::List => env::list_env_records(name.clone()),
                };
            }
            Commands::Call { name } => {
                let project_path = get_current_project_config_path()?;
                let config = load_config(&project_path)?;

                let requests = config.get_requests(&project_path.parent().unwrap());
                println!("name: {name}, req: {:?}", requests);
                return Ok(());
            }
        };
    } else {
        // eprintln!("pass --help to see usage");
        return Err("pass --help to see usage".to_string());
    }
}

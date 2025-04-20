use clap::{Parser, Subcommand};

pub mod env;
pub mod init;
pub mod add;

#[derive(Parser, Debug)]
#[command(
    name = "dotapi",
    about = "An api client testing tool",
    version = "0.0.1"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initializes a dotapi project
    Init { path: Option<String> },
    
    /// Adds a new request to the project
    Add {
        /// Name of the request
        #[arg(short, long)]
        name: String,

        /// HTTP method (GET, POST, PUT, DELETE, etc)
        #[arg(short, long, default_value = "GET")]
        method: String,

        /// The URL for the request
        url: String,

        /// Request query parameters in key=value format
        #[arg(short = 'p', long)]
        params: Vec<String>,

        /// Request headers in key:value format
        #[arg(short = 'H', long = "header")]
        headers: Vec<String>,

        /// Request body as raw text
        #[arg(short = 'B', long = "body")]
        body: Option<String>,

        /// Form data in key=value format
        #[arg(short = 'F', long = "form")]
        form: Vec<String>,

        /// Files to send in key=@path format
        #[arg(short = 'f', long = "file")]
        files: Vec<String>,

        /// Basic auth in username:password format
        #[arg(short = 'a', long = "auth")]
        auth: Option<String>,

        /// Bearer token for authentication
        #[arg(long = "bearer")]
        bearer: Option<String>,

        /// API key for authentication
        #[arg(long = "apikey")]
        api_key: Option<String>,

        /// Header name for API key (default: "Authorization")
        #[arg(long = "apikey-header")]
        api_key_header: Option<String>,
    },
    /// Sets an enviroment variable for the project
    Env {
        /// Name of the enviroment. if not specified the default is used
        #[arg(short, long)]
        name: Option<String>,

        #[command(subcommand)]
        command: EnvSubcommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvSubcommands {
    Set {
        /// The key of the variable
        #[arg(short, long)]
        key: String,

        /// The value for the variable
        value: String,
    },
    UnSet {
        /// The key of the variable
        #[arg(short, long)]
        key: String,
    },
    Delete,
    List,
}

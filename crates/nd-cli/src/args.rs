use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "nativedoctor",
    version,
    about = "A configurable API testing tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a collection
    Init {
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        dir: PathBuf,
        /// Collection name (defaults to the folder name)
        #[arg(long)]
        name: Option<String>,
    },
    /// Run a request file, or every request in a folder
    Run(RunArgs),
    /// Send a one-off request
    Send(SendArgs),
    /// List the requests in a collection
    List {
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        path: PathBuf,
    },
    /// See or clear saved cookies
    Cookies {
        #[command(subcommand)]
        action: CookiesAction,
    },
    /// Validate a collection without using the network
    Check {
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        path: PathBuf,
        /// Environment to check against
        #[arg(short, long)]
        env: Option<String>,
        /// Treat a variable as set: NAME=value
        #[arg(long = "var", value_name = "NAME=VALUE", value_parser = parse_var)]
        vars: Vec<(String, String)>,
    },
}

#[derive(Subcommand)]
pub enum CookiesAction {
    /// List saved cookies, without their values
    List {
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        path: PathBuf,
        /// Environment to use
        #[arg(short, long)]
        env: Option<String>,
    },
    /// Delete saved cookies for one environment
    Clear {
        #[arg(default_value = ".", value_hint = clap::ValueHint::DirPath)]
        path: PathBuf,
        /// Environment to use
        #[arg(short, long)]
        env: Option<String>,
    },
}

#[derive(Args)]
pub struct RunArgs {
    /// Request file or folder
    #[arg(value_hint = clap::ValueHint::AnyPath)]
    pub target: PathBuf,
    /// One-time changes: Header:value, key==query, key=json, key:=raw, field@file
    pub items: Vec<String>,
    #[command(flatten)]
    pub options: Options,
}

#[derive(Args)]
pub struct SendArgs {
    /// [METHOD] URL [ITEM...]. The method defaults to GET, or POST when there's a body.
    #[arg(required = true, value_name = "ARGS")]
    pub args: Vec<String>,
    /// Save the request as a file, then send it. Add --offline to only save.
    #[arg(long, value_name = "FILE")]
    pub save: Option<PathBuf>,
    #[command(flatten)]
    pub options: Options,
}

#[derive(Args)]
pub struct Options {
    /// Environment to use
    #[arg(short, long)]
    pub env: Option<String>,
    /// Set a variable: NAME=value
    #[arg(long = "var", value_name = "NAME=VALUE", value_parser = parse_var)]
    pub vars: Vec<(String, String)>,
    /// Send data items as a URL-encoded form
    #[arg(short, long, conflicts_with = "multipart")]
    pub form: bool,
    /// Send data items as multipart
    #[arg(long)]
    pub multipart: bool,
    /// Raw body: text, @file, or - for stdin
    #[arg(long, value_name = "BODY", allow_hyphen_values = true)]
    pub body: Option<String>,
    /// Don't send or save cookies
    #[arg(long)]
    pub no_cookies: bool,
    /// Exit 3, 4 or 5 on a 3xx, 4xx or 5xx response
    #[arg(long)]
    pub fail: bool,
    /// Follow redirects
    #[arg(short = 'F', long)]
    pub follow: bool,
    #[arg(long, value_name = "N")]
    pub max_redirects: Option<usize>,
    /// Like 500ms, 30s or 2m. 0 disables it.
    #[arg(long, value_name = "TIME")]
    pub timeout: Option<String>,
    /// Skip TLS certificate checks
    #[arg(long)]
    pub insecure: bool,
    /// Build and print the request without sending it
    #[arg(long)]
    pub offline: bool,
    /// What to print: H and B (request), h and b (response), m (timing)
    #[arg(short, long, value_name = "WHAT")]
    pub print: Option<String>,
    /// Print the full exchange, same as --print=HBhbm
    #[arg(short, long)]
    pub verbose: bool,
    /// Print body chunks as they arrive
    #[arg(short = 'S', long)]
    pub stream: bool,
    /// Print a JSON result. With --stream, one JSON event per line.
    #[arg(long)]
    pub json: bool,
    /// Formatting and colours. Defaults to all in a terminal and none in a pipe.
    #[arg(long, value_enum)]
    pub pretty: Option<Pretty>,
}

#[derive(Clone, Copy, PartialEq, ValueEnum)]
pub enum Pretty {
    All,
    None,
}

fn parse_var(raw: &str) -> Result<(String, String), String> {
    return raw
        .split_once('=')
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .ok_or_else(|| format!("expected NAME=value, got `{raw}`"));
}

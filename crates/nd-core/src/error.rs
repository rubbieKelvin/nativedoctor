use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse request file {path}: {source}")]
    ParseYaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("failed to parse request file {path}: {source}")]
    ParseJson {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("unsupported request file format (use .json, .yaml, or .yml): {0}")]
    UnsupportedFormat(PathBuf),

    #[error("missing environment variable '{0}' in template")]
    MissingTemplateVar(String),

    #[error("invalid HTTP request: {0}")]
    InvalidRequest(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Rhai script error: {0}")]
    Rhai(String),

    #[error("post_script not found: {0}")]
    PostScriptNotFound(PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;

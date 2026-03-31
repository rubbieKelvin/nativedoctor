use std::path::PathBuf;

use thiserror::Error;

/// Errors returned by [`crate::Result`].
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

    /// A `${VAR}` reference had no value in [`crate::RuntimeEnv`] or the process environment.
    #[error("missing environment variable '{0}' in template")]
    MissingTemplateVar(String),

    /// Unknown `${!name}` dynamic template function (see `env::dynamic`).
    #[error("unknown dynamic template '{0}'")]
    UnknownDynamicTemplate(String),

    #[error("invalid HTTP request: {0}")]
    InvalidRequest(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Rhai script error: {0}")]
    Rhai(String),

    #[error("post_script not found: {0}")]
    PostScriptNotFound(PathBuf),

    #[error("invalid sequence file {path}: {source}")]
    ParseSequenceYaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("invalid sequence file {path}: {source}")]
    ParseSequenceJson {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("invalid sequence: {0}")]
    InvalidSequence(String),

    #[error("sequence step request file not found: {0}")]
    SequenceStepNotFound(PathBuf),

    #[error("failed to read env file {path}: {source}")]
    EnvFileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid env file {path} line {line}: {message}")]
    EnvFileParse {
        path: PathBuf,
        line: usize,
        message: String,
    },

    #[error("No runtime persist file: {message}")]
    NoRuntimePersistFile { message: String },

    #[error("invalid runtime persist file {path}: {message}")]
    InvalidRuntimePersistFile { path: PathBuf, message: String },
}

/// Convenient alias used across this crate.
pub type Result<T> = std::result::Result<T, Error>;

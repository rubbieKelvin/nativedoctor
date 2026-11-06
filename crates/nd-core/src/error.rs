use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}: {source}", path.display())]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{}: {source}", path.display())]
    Yaml {
        path: PathBuf,
        source: serde_yaml_ng::Error,
    },
    #[error("{0}")]
    Invalid(String),
    #[error("invalid request item `{item}`: {reason}")]
    Item { item: String, reason: String },
    #[error("variable `{0}` isn't declared; add it to the environment or pass --var {0}=...")]
    UnknownVar(String),
    #[error("variable `{0}` is declared but has no value; set it in the environment or pass --var {0}=...")]
    EmptyVar(String),
    #[error("secret `{name}` has no value; set it in the process environment or in {hint}")]
    MissingSecret { name: String, hint: String },
    #[error("unknown dynamic variable `${{!{0}}}`")]
    UnknownDynamic(String),
    #[error("unknown environment `{name}` (available: {available})")]
    UnknownEnv { name: String, available: String },
    #[error("{} is outside the collection; request files can only read files inside it", .0.display())]
    FileOutsideRoot(PathBuf),
    #[error("{0}")]
    Unsupported(String),
    #[error("can't save while keeping the file's formatting: {0}")]
    Unpreserved(String),
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        return Error::Io {
            path: path.into(),
            source,
        };
    }
}

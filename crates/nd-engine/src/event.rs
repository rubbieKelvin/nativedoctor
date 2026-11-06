use std::time::Duration;

use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Event {
    Head {
        status: u16,
        reason: Option<String>,
        version: String,
        headers: Vec<(String, String)>,
    },
    Chunk(Bytes),
    Finished {
        elapsed: Duration,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request timed out after {0:?}")]
    Timeout(Duration),
    #[error("exceeded the limit of {0} redirects")]
    TooManyRedirects(usize),
    #[error("connection failed: {0}")]
    Connect(String),
    #[error("invalid request: {0}")]
    Invalid(String),
    #[error("{0}")]
    Other(String),
    #[error(transparent)]
    Core(#[from] nd_core::Error),
}

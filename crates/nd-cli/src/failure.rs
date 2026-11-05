use std::io;

use nd_core::Redactor;

pub const ERROR: u8 = 1;
pub const TIMEOUT: u8 = 2;
pub const REDIRECTS: u8 = 6;
pub const ASSERTION: u8 = 7;

#[derive(Debug)]
pub struct Failure {
    pub code: u8,
    pub message: String,
}

impl Failure {
    pub fn new(message: impl Into<String>) -> Self {
        return Failure {
            code: ERROR,
            message: message.into(),
        };
    }

    pub fn redacted(mut self, redactor: &Redactor) -> Self {
        self.message = redactor.redact(&self.message);
        return self;
    }
}

impl From<nd_core::Error> for Failure {
    fn from(error: nd_core::Error) -> Self {
        Failure::new(error.to_string())
    }
}

impl From<io::Error> for Failure {
    fn from(error: io::Error) -> Self {
        // The reader went away, as in `| head`. That isn't a failure.
        if error.kind() == io::ErrorKind::BrokenPipe {
            return Failure {
                code: 0,
                message: String::new(),
            };
        }
        return Failure::new(error.to_string());
    }
}

impl From<nd_engine::Error> for Failure {
    fn from(error: nd_engine::Error) -> Self {
        let code = match error {
            nd_engine::Error::Timeout(_) => TIMEOUT,
            nd_engine::Error::TooManyRedirects(_) => REDIRECTS,
            _ => ERROR,
        };
        return Failure {
            code,
            message: error.to_string(),
        };
    }
}

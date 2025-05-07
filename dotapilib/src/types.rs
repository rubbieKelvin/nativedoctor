use std::fmt::Display;

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE,
    TRACE,
    Custom(String),
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Self::GET => "get",
                Self::POST => "post",
                Self::PATCH => "patch",
                Self::DELETE => "delete",
                Self::PUT => "put",
                Self::OPTION => "option",
                Self::TRACE => "trace",
                Self::Custom(method) => method,
            }
        );
    }
}

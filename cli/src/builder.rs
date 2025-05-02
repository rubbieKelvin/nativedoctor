use std::fmt::Display;

pub fn add() {}

pub enum HttpMethod {
    GET,
    OPTION,
    HEAD,
    PUT,
    PATCH,
    POST,
    DELETE,
    Custom(String),
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GET => "get",
            Self::PUT => "put",
            Self::DELETE => "delete",
            Self::HEAD => "head",
            Self::OPTION => "option",
            Self::PATCH => "patch",
            Self::POST => "post",
            Self::Custom(v) => v,
        };

        return write!(f, "{}", s);
    }
}

pub struct RequestBuilder {
    method: HttpMethod,
    url: String,
}

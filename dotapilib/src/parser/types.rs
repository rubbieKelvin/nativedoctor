// decribes a single api file
#[derive(Debug, PartialEq)]
pub struct ApiFile {
    pub blocks: Vec<Blocks>,
}

#[derive(Debug, PartialEq)]
pub enum Blocks {
    Env(EnvBlock),
    Call(CallBlock),
    Request(RequestBlock),
}

#[derive(Debug, PartialEq)]
pub struct EnvBlock {
    pub variables: Vec<EnvVariable>,
}

#[derive(Debug, PartialEq)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub environment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct CallBlock;

#[derive(Debug, PartialEq)]
pub struct RequestBlock {
    pub name: String,
    pub method: Method,
    pub url: String,
    pub config: Option<Vec<ConfigProperty>>,
    pub headers: Option<Vec<Header>>,
    pub query_params: Option<Vec<QueryParam>>,
    pub body: Option<BodyContent>,
    pub post_request_script: Option<String>, // script content as a string
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConfigValue {
    String(String),
    Number(i64),
    Boolean(bool),
    StringArray(Vec<String>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BodyContent {
    Json(String),
    Graphql {
        query: String,
        variables: Option<String>, // TODO: see if we can serialize this. not a priority tho
    },
    Multipart(Vec<MultipartField>),
    Xml(String),
    Text(String),
    FormUrlEncoded(String),
    Raw(String), // generic @body without type
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigProperty {
    pub key: String,
    pub value: ConfigValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MultipartField {
    Field {
        name: String,
        value: String,
    },
    File {
        name: String,
        path: String,
        mime_type: Option<String>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct QueryParam {
    pub name: String,
    pub value: String,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTIONS,
    DELETE,
    TRACE,
    HEAD,
    CONNECT,
}

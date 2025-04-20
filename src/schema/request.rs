use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{constants::REQUEST_FOLDER, utils::sanitize_filename};

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    ApiKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BearerAuth {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKeyAuth {
    pub key: String,
    pub in_header: bool,
    pub header_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Auth {
    Basic(BasicAuth),
    Bearer(BearerAuth),
    ApiKey(ApiKeyAuth),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestBodyType {
    TEXT,
    FORM,
    FILE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DismissableStringValue {
    pub value: String,
    pub enabled: bool,
}

impl DismissableStringValue {
    pub fn new(value: String) -> Self {
        Self {
            value,
            enabled: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    pub key: String,
    pub value: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestBodyData {
    Text(String),
    Form(Vec<FormData>),
    File(Vec<FileData>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub body_type: RequestBodyType,
    pub data: RequestBodyData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: HttpMethod,
    pub auth: Option<Auth>,
    pub params: HashMap<String, DismissableStringValue>,
    pub headers: HashMap<String, DismissableStringValue>,
    pub body: Option<RequestBody>,
}

impl Request {
    fn parse_key_value(input: &str, separator: char) -> Option<(String, String)> {
        let parts: Vec<&str> = input.split(separator).collect();
        if parts.len() == 2 {
            Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
        } else {
            None
        }
    }

    fn parse_auth(auth_str: &str) -> Option<Auth> {
        let parts: Vec<&str> = auth_str.split(':').collect();
        if parts.len() == 2 {
            Some(Auth::Basic(BasicAuth {
                username: parts[0].to_string(),
                password: parts[1].to_string(),
            }))
        } else {
            None
        }
    }

    pub fn parse_from_args(
        name: String,
        url: String,
        method: String,
        params: Vec<String>,
        headers: Vec<String>,
        body: Option<String>,
        form: Vec<String>,
        files: Vec<String>,
        auth: Option<String>,
        bearer: Option<String>,
        api_key: Option<String>,
        api_key_header: Option<String>,
    ) -> Request {
        // Parse HTTP method
        let method = match method.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            _ => HttpMethod::GET, // Default to GET for unknown methods
        };

        // Parse query parameters
        let mut params_map = HashMap::new();
        for param in params {
            if let Some((key, value)) = Request::parse_key_value(&param, '=') {
                params_map.insert(key, DismissableStringValue::new(value));
            }
        }

        // Parse headers
        let mut headers_map = HashMap::new();
        for header in headers {
            if let Some((key, value)) = Request::parse_key_value(&header, ':') {
                headers_map.insert(key, DismissableStringValue::new(value));
            }
        }

        // Handle authentication
        let auth = if let Some(auth_str) = auth {
            Request::parse_auth(&auth_str)
        } else if let Some(token) = bearer {
            Some(Auth::Bearer(BearerAuth { token }))
        } else if let Some(key) = api_key {
            Some(Auth::ApiKey(ApiKeyAuth {
                key,
                in_header: true,
                header_name: api_key_header.unwrap_or_else(|| "Authorization".to_string()),
            }))
        } else {
            None
        };

        // Handle request body
        let body = if !form.is_empty() {
            // Handle form data
            let form_data: Vec<FormData> = form
                .iter()
                .filter_map(|f| Request::parse_key_value(f, '='))
                .map(|(key, value)| FormData {
                    key,
                    value,
                    enabled: true,
                })
                .collect();

            if !form_data.is_empty() {
                Some(RequestBody {
                    body_type: RequestBodyType::FORM,
                    data: RequestBodyData::Form(form_data),
                })
            } else {
                None
            }
        } else if !files.is_empty() {
            // Handle file uploads
            let file_data: Vec<FileData> = files
                .iter()
                .filter_map(|f| {
                    if let Some((key, path)) = Request::parse_key_value(f, '=') {
                        if path.starts_with('@') {
                            Some(FileData {
                                name: key,
                                path: path[1..].to_string(), // Remove @ prefix
                                enabled: true,
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            if !file_data.is_empty() {
                Some(RequestBody {
                    body_type: RequestBodyType::FILE,
                    data: RequestBodyData::File(file_data),
                })
            } else {
                None
            }
        } else if let Some(body_str) = body {
            // Handle raw text body
            Some(RequestBody {
                body_type: RequestBodyType::TEXT,
                data: RequestBodyData::Text(body_str),
            })
        } else {
            None
        };

        Request {
            name,
            url,
            method,
            auth,
            params: params_map,
            headers: headers_map,
            body,
        }
    }

    pub fn save(&self, project_root: &Path) -> Result<(), String> {
        let content = serde_yaml::to_string(&self).map_err(|e| e.to_string())?;
        let path = Path::new(project_root.to_str().unwrap()).join(REQUEST_FOLDER);

        if !path.try_exists().unwrap() {
            fs::create_dir(&path).unwrap();
        }

        let sanitized_name = format!(
            "{}_{}_{}.yaml",
            match self.method {
                HttpMethod::GET => "get",
                HttpMethod::HEAD => "head",
                HttpMethod::POST => "post",
                HttpMethod::PUT => "put",
                HttpMethod::PATCH => "patch",
                HttpMethod::DELETE => "delete",
                HttpMethod::OPTIONS => "options",
            },
            sanitize_filename(&self.name),
            sanitize_filename(&self.url)
        );

        let path = path.join(&sanitized_name);

        let mut file = File::create(path).map_err(|e| e.to_string())?;

        file.write_all(content.as_bytes()).unwrap();
        return Ok(());
    }
}

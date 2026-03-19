/// Key-value pair for params/headers. Used for JSON in/out of read/write_resource_file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
}

/// HTTP resource as stored in .request.yaml. Deserialize from YAML, serialize to JSON for frontend.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct HttpResourceFile {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    pub id: String,
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub params: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    pub headers: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub auth: Option<serde_json::Value>,
    #[serde(default)]
    pub pre_request_script: String,
    #[serde(default)]
    pub post_request_script: String,
    #[serde(default)]
    pub settings: HttpRequestSettings,
}

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
pub struct HttpRequestSettings {
    #[serde(default)]
    pub max_number_of_redirects: Option<i64>,
    #[serde(default)]
    pub timeout: Option<u64>,
    /// Use HTTP/2 when possible (default true). Set false to force HTTP/1.1.
    #[serde(default)]
    pub use_http2: Option<bool>,
}

/// One cookie parsed from a Set-Cookie response header. Uses the `cookie` crate for parsing.
#[derive(serde::Serialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: String,
    pub secure: bool,
    pub same_site: String,
}

impl From<String> for Cookie {
    /// Parses a Set-Cookie header value using the `cookie` crate (e.g. "name=value; Path=/; Domain=...; Secure").
    /// On parse error, returns a Cookie with empty name and the raw header as value so the frontend still sees it.
    fn from(header: String) -> Self {
        match cookie::Cookie::parse(header.trim()) {
            Ok(c) => {
                let expires = c
                    .expires_datetime()
                    .map(|t| t.to_string())
                    .unwrap_or_default();
                let same_site = c
                    .same_site()
                    .map(|s| format!("{:?}", s))
                    .unwrap_or_default();
                Cookie {
                    name: c.name().to_string(),
                    value: c.value().to_string(),
                    domain: c.domain().map(str::to_string).unwrap_or_default(),
                    path: c.path().map(str::to_string).unwrap_or_default(),
                    expires,
                    secure: c.secure() == Some(true),
                    same_site,
                }
            }
            Err(_) => Cookie {
                name: String::new(),
                value: header,
                domain: String::new(),
                path: String::new(),
                expires: String::new(),
                secure: false,
                same_site: String::new(),
            },
        }
    }
}

#[derive(serde::Serialize)]
pub struct HttpResponse {
    pub url: String,
    pub status: u16,
    pub headers: Vec<[String; 2]>,
    pub body: String,
    pub duration_ms: u64,
    pub size: u64,
    /// Response coolies
    pub cookies: Vec<Cookie>,
    /// HTTP version used for the response (e.g. "HTTP/1.1", "HTTP/2").
    pub http_version: String,
}

/// Strips \r and \n from header name/value so reqwest doesn't return builder error.
fn sanitize_header(s: &str) -> String {
    s.replace(['\r', '\n'], " ").trim().to_string()
}

use tracing::{error, info};

impl HttpResourceFile {
    /// Executes the HTTP request and returns the response.
    pub async fn call(&self, client: &reqwest::Client) -> Result<HttpResponse, String> {
        let url_str = self.url.trim();

        if url_str.is_empty() {
            return Err("URL is required".to_string());
        }

        // Default to https:// when no scheme is present (e.g. "google.com" -> "https://google.com").
        let url_str = if url_str.contains("://") {
            url_str.to_string()
        } else {
            format!("https://{}", url_str)
        };

        let url = reqwest::Url::parse(&url_str).map_err(|e| {
            let msg = format!("Invalid URL: {}", e);
            error!(error = %msg, url = %url_str, "HttpResourceFile::call: URL parse failed");
            msg
        })?;

        // Parse the HTTP method (GET, POST, etc.) and start timing.
        let method = self
            .method
            .trim()
            .parse::<reqwest::Method>()
            .map_err(|e| {
                let msg = format!("Invalid method: {}", e);
                error!(error = %msg, method = %self.method, "HttpResourceFile::call: method parse failed");
                msg
            })?;
        let start = std::time::Instant::now();

        let enabled_params_count = self
            .params
            .as_ref()
            .map(|ps| ps.iter().filter(|p| p.enabled).count())
            .unwrap_or(0);

        let mut enabled_param_keys: Vec<String> = Vec::new();
        if let Some(ref params) = self.params {
            for p in params.iter().filter(|p| p.enabled) {
                enabled_param_keys.push(p.key.clone());
                if enabled_param_keys.len() >= 10 {
                    break;
                }
            }
        }

        let enabled_headers_count = self
            .headers
            .as_ref()
            .map(|hs| hs.iter().filter(|h| h.enabled).count())
            .unwrap_or(0);

        let mut enabled_header_keys: Vec<String> = Vec::new();
        if let Some(ref headers) = self.headers {
            for h in headers.iter().filter(|h| h.enabled) {
                enabled_header_keys.push(h.key.clone());
                if enabled_header_keys.len() >= 10 {
                    break;
                }
            }
        }

        info!(
            method = %method,
            url = %url_str,
            enabled_params = enabled_params_count,
            enabled_param_keys = ?enabled_param_keys,
            enabled_headers = enabled_headers_count,
            enabled_header_keys = ?enabled_header_keys,
            "HttpResourceFile::call: preparing request"
        );

        // Build the request: method + URL.
        let mut req = client.request(method, url.clone());

        // Add query parameters from params (only enabled ones). reqwest merges these with any existing query in the URL.
        if let Some(ref params) = self.params {
            let query: Vec<(&str, &str)> = params
                .iter()
                .filter(|p| p.enabled)
                .map(|p| (p.key.as_str(), p.value.as_str()))
                .collect();
            if !query.is_empty() {
                req = req.query(&query);
            }
        }

        // Add request headers (only enabled ones). Sanitize name/value to avoid builder error from invalid characters.
        if let Some(ref headers) = self.headers {
            for h in headers.iter().filter(|h| h.enabled) {
                let name = sanitize_header(h.key.as_str());
                let value = sanitize_header(h.value.as_str());
                if !name.is_empty() {
                    req = req.header(name.as_str(), value);
                }
            }
        }

        // Set request body. If body is a JSON string we use it as-is; if it's an object we serialize it to JSON.
        if let Some(ref body) = self.body {
            let body_str = if body.is_string() {
                body.as_str().unwrap_or("").to_string()
            } else {
                serde_json::to_string(body).unwrap_or_default()
            };
            if !body_str.is_empty() {
                let body_kind = if body.is_string() { "string" } else { "json" };
                info!(
                    body_kind = body_kind,
                    body_len = body_str.len(),
                    "HttpResourceFile::call: using request body"
                );
                req = req.body(body_str);
            }
        }

        // Send the request and fail fast on connection/send errors.
        let res = req.send().await.map_err(|e| {
            let msg = e.to_string();
            error!(error = %msg, "HttpResourceFile::call: send failed");
            msg
        })?;

        // Read response metadata (must be done before consuming the body).
        let status = res.status().as_u16();
        let url = res.url().to_string();
        let http_version = match format!("{:?}", res.version()).as_str() {
            "Http09" => "HTTP/0.9",
            "Http10" => "HTTP/1.0",
            "Http11" => "HTTP/1.1",
            "Http2" => "HTTP/2",
            "Http3" => "HTTP/3",
            other => other,
        }
        .to_string();
        let headers: Vec<[String; 2]> = res
            .headers()
            .iter()
            .map(|(n, v)| [n.as_str().to_string(), v.to_str().unwrap_or("").to_string()])
            .collect();

        // Parse Set-Cookie headers into Cookie structs (name, value, domain, path, etc.) before consuming the body.
        let cookies: Vec<Cookie> = res
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(|s| Cookie::from(s.to_string()))
            .collect();

        // Consume the response body (this can only be done once).
        let body = res.text().await.map_err(|e| {
            let msg = e.to_string();
            error!(error = %msg, "HttpResourceFile::call: reading response body failed");
            msg
        })?;
        let size = body.len() as u64;
        let duration_ms = start.elapsed().as_millis() as u64;

        info!(
            status = status,
            duration_ms = duration_ms,
            response_headers = headers.len(),
            response_body_size = size,
            http_version = %http_version,
            "HttpResourceFile::call: response received"
        );

        Ok(HttpResponse {
            url,
            status,
            headers,
            body,
            duration_ms,
            size,
            cookies,
            http_version,
        })
    }
}

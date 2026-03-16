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
    pub cookies: Vec<Cookie>,
}

impl HttpResourceFile {
    /// Executes the HTTP request and returns the response.
    pub async fn call(&self, client: &reqwest::Client) -> Result<HttpResponse, String> {
        // Parse the HTTP method (GET, POST, etc.) and start timing.
        let method = self
            .method
            .parse::<reqwest::Method>()
            .map_err(|e| e.to_string())?;
        let start = std::time::Instant::now();

        // Build the request: method + URL.
        let mut req = client.request(method, self.url.as_str());

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

        // Add request headers (only enabled ones).
        if let Some(ref headers) = self.headers {
            for h in headers.iter().filter(|h| h.enabled) {
                req = req.header(h.key.as_str(), h.value.as_str());
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
                req = req.body(body_str);
            }
        }

        // Send the request and fail fast on connection/send errors.
        let res = req.send().await.map_err(|e| e.to_string())?;

        // Read response metadata (must be done before consuming the body).
        let status = res.status().as_u16();
        let url = res.url().to_string();
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
        let body = res.text().await.map_err(|e| e.to_string())?;
        let size = body.len() as u64;
        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(HttpResponse {
            url,
            status,
            headers,
            body,
            duration_ms,
            size,
            cookies,
        })
    }
}

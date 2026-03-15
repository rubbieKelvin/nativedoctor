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
    pub timeout: Option<i64>,
}

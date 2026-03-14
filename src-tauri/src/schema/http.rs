/// Key-value pair for params/headers. Used for JSON in/out of read/write_resource_file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default)]
    pub description: Option<String>,
}

/// HTTP resource as stored in .request.yaml. Deserialize from YAML, serialize to JSON for frontend.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpResourceFile {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(rename = "type", default = "default_http_type")]
    pub resource_type: String,
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
}

fn default_http_type() -> String {
    "http".into()
}

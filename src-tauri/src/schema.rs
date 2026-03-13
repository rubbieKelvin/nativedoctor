#[derive(serde::Deserialize, serde::Serialize)]
pub struct EnvSource {
    pub name: String,
    pub path: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativedoctorJson {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_sources: Option<Vec<EnvSource>>,
}

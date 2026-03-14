/// Sequence node as stored in .sequence.yaml.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SequenceNode {
    pub id: String,
    pub resource_id: String,
    pub resource_type: String,
}

/// Sequence resource as stored in .sequence.yaml.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SequenceResourceFile {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(rename = "type", default = "default_sequence_type")]
    pub resource_type: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub flow: Option<Vec<SequenceNode>>,
}

fn default_sequence_type() -> String {
    "sequence".into()
}

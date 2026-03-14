/// Folder resource as stored in .folder.yaml.
/// Children are not persisted; the tree is rebuilt from folder_id on load.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FolderResourceFile {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub folder_id: Option<String>,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: Option<i64>,
    #[serde(default)]
    pub name: String,
}

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct UserSchema {
    pub name: String,
    #[serde(default)]
    pub email: String,
}

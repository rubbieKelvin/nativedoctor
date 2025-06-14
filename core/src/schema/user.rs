use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub struct UserSchema {
    pub name: String,
    #[serde(default)]
    pub email: String,
}

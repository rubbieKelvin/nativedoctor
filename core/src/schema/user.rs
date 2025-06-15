use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct UserSchema {
    pub name: String,
    #[serde(default)]
    pub email: String,
}

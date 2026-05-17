use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecentProject {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub last_opened_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativedoctorProject {
    pub name: String,
    pub version: String,
    pub requests: Vec<String>,
}

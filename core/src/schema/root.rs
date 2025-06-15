use crate::schema::{
    env::EnvironmentVariableSchema, meta::MetaSchema, project::ProjectDefinationSchema,
    request::RequestSchema,
};
use anyhow::{bail, Context, Result};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
use tokio::io::{AsyncReadExt, BufReader};

/// Represents the entire API test file structure.
#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Clone)]
pub struct RootSchema {
    #[serde(default)] // Make imports optional
    pub imports: Vec<String>,
    #[serde(default)] // Make env optional
    pub env: HashMap<String, EnvironmentVariableSchema>,
    #[serde(default)] // Make requests optional
    pub requests: HashMap<String, RequestSchema>,
    #[serde(default)] // Make calls optional
    pub calls: HashMap<String, Vec<String>>,
    /// Project defination, just for more information
    #[serde(default)]
    pub project: Option<ProjectDefinationSchema>,
    #[serde(skip, default)]
    pub meta: Option<MetaSchema>,
}

impl RootSchema {
    /// Load root schema from path, as-is.
    /// path is the file path to load. this should be an absolute path, no relative
    /// caller file is the file that initiated the run.
    pub async fn load(path: &Path) -> Result<Self> {
        tracing::info!("Opening file: {:?}", path);

        if !path.is_absolute() {
            bail!("Path to load must be an absolute path");
        }

        let file = tokio::fs::File::open(&path)
            .await
            .context(format!("Failed to open file"))?;

        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;

        let mut schema: RootSchema = serde_yaml::from_str(&content).context("Parse Error")?;

        // fill meta
        let meta = MetaSchema::new()
            .set_filepath(Some(path.to_path_buf()))
            .set_main_file(Some(path.to_path_buf()));

        schema.meta = Some(meta);

        return Ok(schema);
    }

    pub fn to_string(&self) -> Result<String> {
        return serde_yaml::to_string(&self).context("Could not serialize root schema");
    }
}

use crate::schema::{
    env::EnvironmentVariableSchema, meta::MetaSchema, project::ProjectDefinationSchema,
    request::RequestSchema,
};
use anyhow::{bail, Context, Result};
use async_recursion::async_recursion;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use tokio::{
    fs::canonicalize,
    io::{AsyncReadExt, BufReader},
};
use uuid::Uuid;

/// Represents the entire API test file structure.
#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
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

#[derive(Deserialize, PartialEq, Clone)]
pub struct LoadedRootObject {
    pub id: Uuid,
    pub schema: RootSchema,
    pub imports: Vec<Box<LoadedRootObject>>,
    pub path: PathBuf
}

impl RootSchema {
    /// Load root schema from path, as-is.
    /// path is the file path to load. this should be an absolute path, no relative
    /// caller file is the file that initiated the run.
    pub async fn load(path: &Path) -> Result<Self> {
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

    /// Loads root schema from path, then loads all imports from path too
    /// This will also watch out for circular-imports
    /// caller_file is the file that began the run
    /// import trace is the a list of file paths that led to the current file beign loaded (for circular dep check)
    #[async_recursion]
    async fn _load_recursive(
        path: &Path,
        caller_file: Option<PathBuf>,
        mut import_trace: HashSet<String>,
    ) -> Result<LoadedRootObject> {
        // check for circular dep
        let str_path = path
            .to_str()
            .context("Cannot convert import path to string")?
            .to_string();
        if import_trace.contains(&str_path) {
            bail!("Circular dependency detected for import: {}", &str_path);
        } else {
            import_trace.insert(str_path);
        }

        // create schema for this path
        let mut root = RootSchema::load(path).await?;
        let mut imports = vec![];

        // set the actual caller file, since we're recursively importing
        if caller_file.is_some() {
            if let Some(meta) = root.meta {
                root.meta = Some(meta.set_main_file(caller_file.clone()));
            }
        }

        // compute working dir
        let working_dir = match &caller_file {
            Some(p) => {
                let parent = p.parent().context("Could not get file parent")?;
                parent.to_path_buf()
            }
            None => path.to_path_buf(),
        };

        // load imports
        for import_path in root.imports.iter() {
            let path = Path::new(&import_path);
            let path = canonicalize(working_dir.join(path)).await?;

            let loaded_object = RootSchema::_load_recursive(
                path.as_path(),
                caller_file.clone(),
                import_trace.clone(),
            )
            .await?;
            imports.push(Box::new(loaded_object));
        }

        return Ok(LoadedRootObject {
            id: Uuid::new_v4(),
            schema: root,
            imports,
            path: path.to_path_buf()
        });
    }

    pub async fn load_recursive(path: &Path) -> Result<LoadedRootObject> {
        // path must be absolute
        if !path.is_absolute() {
            bail!("Path to recursive load must be absolute");
        }
        return RootSchema::_load_recursive(path, None, HashSet::new()).await;
    }
}

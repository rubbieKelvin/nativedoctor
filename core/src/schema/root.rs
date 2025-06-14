use anyhow::{Context, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{canonicalize, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::schema::{
    env::EnvironmentVariableSchema, meta::MetaSchema, project::ProjectDefinationSchema,
    request::RequestSchema,
};

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

impl RootSchema {
    pub fn new(path: &Path, mut wd: Option<PathBuf>) -> Result<Self> {
        let resultant_file_path = match &wd {
            Some(p) => canonicalize(p.join(path)).context("Could not canonicalize path")?,
            None => path.to_path_buf(),
        };

        if wd.is_none() {
            let rfp = resultant_file_path.clone();
            wd = Some(
                rfp.parent()
                    .context("Cannot get file parent")?
                    .to_path_buf(),
            );
        }

        let working_directory = wd.unwrap();

        let file = File::open(&resultant_file_path).context(format!("Failed to open file"))?;

        let reader = BufReader::new(file);
        let mut schema: RootSchema = serde_yaml::from_reader(reader).context("Parse Error")?;

        // fill meta
        let meta = MetaSchema::new()
            .add_filepath(Some(resultant_file_path))
            .add_wd(Some(working_directory));

        schema.meta = Some(meta);

        return Ok(schema);
    }

    pub fn get_all_requests(&self) -> Result<Vec<(Option<PathBuf>, String, RequestSchema)>> {
        // Go through this root schema and return the requests.
        // mind there's "imports". we'd need to loop though the imports and load all the requests from there too.
        let mut requests = vec![];
        // let working_dir = self
        //     .meta
        //     .clone()
        //     .map(|m| m.working_directory)
        //     .unwrap_or(None);
        let working_dir = self.get_working_dir();

        fn traverse(
            root: RootSchema,
            container: &mut Vec<(Option<PathBuf>, String, RequestSchema)>,
            working_dir: &PathBuf,
        ) -> Result<()> {
            for (name, request) in root.requests {
                container.push((
                    root.meta.clone().map(|m| m.filepath).unwrap_or(None),
                    name,
                    request,
                ));
            }

            for path in root.imports {
                let path = Path::new(&path);
                let schema = RootSchema::new(path, Some(working_dir.clone()))?;
                traverse(schema, container, working_dir)?;
            }

            return Ok(());
        }

        traverse(self.clone(), &mut requests, &working_dir)?;

        return Ok(requests);
    }

    pub fn get_working_dir(&self) -> PathBuf {
        return self
            .meta
            .clone()
            .map(|m| m.working_directory.context("No working dir").unwrap())
            .unwrap();
    }
}

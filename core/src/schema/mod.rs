/// This holds the schema for the native doctor file.
pub mod env;
pub mod meta;
pub mod project;
pub mod request;
pub mod request_body;
pub mod request_config;
pub mod request_script;
pub mod root;
pub mod user;

pub mod file_object {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use anyhow::{bail, Context, Result};
    use async_recursion::async_recursion;
    use serde::Deserialize;
    use tokio::fs::canonicalize;
    use tokio_stream::StreamExt;
    use uuid::Uuid;

    use crate::schema::{request::RequestSchema, root::RootSchema};

    #[derive(Deserialize, PartialEq, Clone, Debug)]
    pub struct LoadedRootObject {
        pub id: Uuid,
        pub schema: RootSchema,
        pub imports: Vec<Box<LoadedRootObject>>,
        pub path: PathBuf,
    }

    #[derive(Deserialize, PartialEq, Clone)]
    pub struct LoadedRequestObject {
        pub id: Uuid,
        pub name: String,
        pub request: RequestSchema,
        pub path: PathBuf,
    }

    impl RootSchema {
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
                // if there's a caller file, let's use the folder where the file resides
                Some(p) => {
                    let parent = p.parent().context("Could not get file parent")?;
                    parent.to_path_buf()
                }
                // else, let's use the folder where this file resides
                None => path
                    .parent()
                    .context("Could not get parent path")?
                    .to_path_buf(),
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
                path: path.to_path_buf(),
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

    impl LoadedRootObject {
        #[async_recursion]
        pub async fn dump_requests(&self, buffer: &mut Vec<LoadedRequestObject>) {
            let schema = self.schema.clone();
            let requests = &mut schema.requests.iter();
            let imports = &mut self.imports.clone();

            let mut request_stream = tokio_stream::iter(requests);
            let mut imports_stream = tokio_stream::iter(imports);

            // load up requests
            while let Some((name, request)) = request_stream.next().await {
                buffer.push(LoadedRequestObject {
                    id: Uuid::new_v4(),
                    name: name.clone(),
                    request: request.clone(),
                    path: self.path.clone(),
                });
            }

            // load up imports
            while let Some(imported_object) = imports_stream.next().await {
                imported_object.dump_requests(buffer).await;
            }
        }
    }
}

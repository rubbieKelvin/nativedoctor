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

pub mod node {
    use anyhow::Context;
    use std::path::Path;

    use crate::schema::root::RootSchema;

    /// Checks a native doctor file to see if it has circular imports
    /// What makes a circular import? if a file's dependency stack falls back to
    /// some already loaded dependency in that stack, then there's a circular import
    // pub fn has_circular_imports(path: &Path) -> Result<bool, Error> {
    //     let schema = RootSchema::new(path)?;
    //     return Ok(false);
    // }

    // We'd use this for other things such such as request dependency checks
    trait Node<T: Clone + PartialEq> {
        fn get_node_value(&self) -> T;
        fn get_children(&self) -> Vec<Box<impl Node<T>>>;

        fn has_circular_imports(&self, trace: Option<Vec<T>>) -> bool {
            let mut trace = trace.unwrap_or_else(|| Vec::new());

            let value = self.get_node_value();

            if trace.contains(&value) {
                return true;
            }

            trace.push(value);

            for child in self.get_children() {
                let report = child.has_circular_imports(Some(trace.clone()));
                if report {
                    return true;
                }
            }
            return false;
        }
    }

    impl Node<String> for RootSchema {
        fn get_node_value(&self) -> String {
            // return self._file.clone().unwrap();
            let path = self
                .meta
                .clone()
                .map(|f| f.filepath.context("No filepath in meta").unwrap())
                .unwrap();
            return path.to_str().unwrap().to_string();
        }

        fn get_children(&self) -> Vec<Box<impl Node<String>>> {
            return self
                .imports
                .iter()
                .filter_map(|import_path| {
                    let path = Path::new(import_path);
                    match RootSchema::new(path, Some(self.get_working_dir())) {
                        Ok(schema) => Some(Box::new(schema)),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<Box<RootSchema>>>();
        }
    }
}

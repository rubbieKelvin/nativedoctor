use std::path::Path;

use crate::{constants::EXTENSION_REQUEST_FILE_YAML, schemas::request::RequestSchema, utils::slugify};

/// Given the name of the request a single request file in the specified directory
pub fn create_request_file<S: AsRef<str>>(name: S, path: &Path) -> Result<(), anyhow::Error> {
    let filename = format!("{}.{}", slugify(&name), EXTENSION_REQUEST_FILE_YAML);
    let filepath = path.join(filename);
    let content_schema = RequestSchema::example(name.as_ref().to_string());
    content_schema.save_to_path(&filepath)?;
    Ok(())
}

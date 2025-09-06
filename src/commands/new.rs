use std::{collections::HashMap, fs::create_dir_all, path::Path};

use crate::{
    constants::{EXTENSION_PROJECT_FILE_YAML, EXTENSION_REQUEST_FILE_YAML},
    schemas::{project::ProjectSchema, request::RequestSchema},
    utils::slugify,
};

/// Given the name of the request a single request file in the specified directory
pub fn create_request_file<S: AsRef<str>>(name: S, path: &Path) -> Result<(), anyhow::Error> {
    let filename = format!("{}.{}", slugify(&name), EXTENSION_REQUEST_FILE_YAML);
    let filepath = path.join(filename);
    let content_schema = RequestSchema::example(name.as_ref().to_string());
    content_schema.save_to_path(&filepath)?;
    return Ok(());
}

/// Create a project folder
pub fn create_project_folder<S: AsRef<str>>(name: S, path: &Path) -> Result<(), anyhow::Error> {
    let slug_name = slugify(&name);
    let path = path.join(&slug_name);

    create_dir_all(&path)?;

    let project_filename = format!("main.{}", EXTENSION_PROJECT_FILE_YAML);
    let project_filepath = path.join(project_filename);

    let project = ProjectSchema {
        name: name.as_ref().to_string(),
        description: Some(format!("API calls for {}", name.as_ref())),
        sequence: Some(HashMap::from_iter(vec![(
            String::from("main"),
            vec![format!("./ping.{}", EXTENSION_REQUEST_FILE_YAML)],
        )])),
        ..Default::default()
    };

    project.save_to_path(&project_filepath)?;
    create_request_file("ping", &path)?;
    return Ok(());
}

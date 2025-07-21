use crate::schema::{
    calls::CallSchema,
    project::ProjectDefinationSchema,
    roots::{ProjectRootSchema, RequestRootSchema},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub mod schema;
#[cfg(test)]
mod tests;

pub const REQUEST_FOLDER_NAME: &str = "requests";
pub const ENVIRONMENT_FOLDER_NAME: &str = "environments";
pub const EXTENSION_FOR_REQUEST: &str = "nd";
pub const EXTENSION_FOR_ENVIRONMENT: &str = "nd-env";
pub const EXTENSION_FOR_PROJECT: &str = "nd-project";

pub fn create_project_template(name: &str) -> (ProjectRootSchema, Vec<RequestRootSchema>) {
    let name = if name.len() == 0 {
        "Untitled".to_string()
    } else {
        name.to_string()
    };

    let project_schema = ProjectRootSchema {
        project: ProjectDefinationSchema {
            name,
            description: "Native doctor project".to_string(),
            version: Some("0.1.0".to_string()),
        },
        calls: CallSchema {
            main: vec!["hello".to_string()],
            overrides: HashMap::new(),
        },
        ..Default::default()
    };

    let hello_request = RequestRootSchema {
        method: "GET".to_string(),
        url: "{{baseurl}}/get".to_string(),
        ..Default::default()
    };

    return (project_schema, vec![hello_request]);
}

// Initializes a new project at path
pub fn init(name: &str, path: &Path) -> anyhow::Result<PathBuf> {
    // Create schemas
    let (project_schema, requests) = create_project_template(name);
    let hello_request = requests[0].clone();

    // create the project file
    let project_path = path.join(format!(".{EXTENSION_FOR_PROJECT}"));
    let request_folder = path.join(REQUEST_FOLDER_NAME);
    let home_folder = request_folder.join(format!("hello.{EXTENSION_FOR_REQUEST}").to_string());

    // create
    std::fs::write(
        &project_path,
        serde_yaml::to_string(&project_schema).unwrap(),
    )?;
    std::fs::create_dir(request_folder)?;
    std::fs::write(&home_folder, serde_yaml::to_string(&hello_request).unwrap())?;

    return Ok(project_path.to_path_buf());
}

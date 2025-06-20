use crate::schema::{
    calls::CallSchema,
    env::EnvironmentVariableSchema,
    project::ProjectDefinationSchema,
    roots::{ProjectRootSchema, RequestRootSchema},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub mod schema;
pub mod fs;
#[cfg(test)]
mod tests;

// Initializes a new project at path
pub async fn init(name: &str, path: &Path) -> anyhow::Result<PathBuf> {
    // Create schemas
    let mut env = HashMap::<String, EnvironmentVariableSchema>::new();
    env.insert(
        "baseurl".to_string(),
        EnvironmentVariableSchema::new(
            serde_yaml::Value::String("https://httpbin.org".to_string()),
            vec![(
                "dev".to_string(),
                serde_yaml::Value::String("http://localhost:8080".to_string()),
            )],
        ),
    );

    let project_schema = ProjectRootSchema {
        project: ProjectDefinationSchema {
            name: name.to_string(),
            description: "Native doctor project".to_string(),
            version: Some("0.1.0".to_string()),
        },
        env,
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

    // create the project file
    let project_path = path.join(format!(".nd-project"));
    let request_folder = path.join("requests");
    let home_folder = request_folder.join("hello.nd".to_string());

    // create
    tokio::fs::write(
        &project_path,
        serde_yaml::to_string(&project_schema).unwrap(),
    )
    .await?;
    tokio::fs::create_dir(request_folder).await?;
    tokio::fs::write(&home_folder, serde_yaml::to_string(&hello_request).unwrap()).await?;

    return Ok(project_path.to_path_buf());
}

use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::info;

use crate::constants;
use crate::schema::env::EnvironmentVariableSchema;
use crate::schema::project::ProjectDefinationSchema;
use crate::schema::request::RequestSchema;
use crate::schema::root::RootSchema;

/// Initializes a new Rustle project at the given path with the specified name.
/// Creates the necessary folder structure and initial YAML files.
///
/// Arguments:
/// * `path`: The directory where the project folder should be created.
/// * `name`: The name of the new project folder and main project file.
#[deprecated]
pub fn init_project<P: AsRef<Path>>(path: P, name: &str) -> Result<()> {
    let project_root = path.as_ref().join(name);

    if project_root.exists() {
        anyhow::bail!("Folder already exists...");
    }

    let requests_dir = project_root.join("requests");
    let calls_dir = project_root.join("calls");

    // 1. Create the main project directory
    fs::create_dir_all(&project_root)
        .with_context(|| format!("Failed to create project directory: {:?}", project_root))?;

    // 2. Create subdirectories
    fs::create_dir_all(&requests_dir)
        .with_context(|| format!("Failed to create requests directory: {:?}", requests_dir))?;
    fs::create_dir_all(&calls_dir)
        .with_context(|| format!("Failed to create calls directory: {:?}", calls_dir))?;

    // 3. Define file paths
    let main_file_name = "project.rt.yaml".to_string();
    let main_file_path = project_root.join(&main_file_name);
    let env_file_path = project_root.join("env.rt.yaml");
    let request_file_path = requests_dir.join("request-01.rt.yaml");
    let calls_file_path = calls_dir.join("init.rt.yaml");

    // 4. Define initial file contents
    let main_file_content = format!(
        r#"
project:
  name: "{}"

imports:
  - env.rt.yaml
  - requests/request-01.rt.yaml
  - calls/init.rt.yaml
"#,
        main_file_name
    );

    let env_file_content = r#"# Environment variables for the project
env:
  # Define your environment variables here
  # Remember all values must be strings
  base_url:
    default: "https://httpbin.org"
"#;

    let request_file_content = r#"requests:
  # A simple example GET request
  Ping:
    method: GET
    url: "{{base_url}}/get" # Uses the base_url from env.api.yaml
    headers:
      Accept: application/json
    # Optional: Add a post-execution script
    # script:
    #   post_request: |
    #     log("Ping successful!");
"#;

    let calls_file_content = r#"# Initial Call Sequences

calls:
  # A simple sequence to run the Ping request
  init:
    - Ping # Calls the Ping request defined in requests/request-01.api.yaml
"#;

    // 5. Write the content to the files
    let mut main_file = fs::File::create(&main_file_path)
        .with_context(|| format!("Failed to create main project file: {:?}", main_file_path))?;
    main_file.write_all(main_file_content.as_bytes())?;

    let mut env_file = fs::File::create(&env_file_path)
        .with_context(|| format!("Failed to create env file: {:?}", env_file_path))?;
    env_file.write_all(env_file_content.as_bytes())?;

    let mut request_file = fs::File::create(&request_file_path)
        .with_context(|| format!("Failed to create request file: {:?}", request_file_path))?;
    request_file.write_all(request_file_content.as_bytes())?;

    let mut calls_file = fs::File::create(&calls_file_path)
        .with_context(|| format!("Failed to create calls file: {:?}", calls_file_path))?;
    calls_file.write_all(calls_file_content.as_bytes())?;

    info!(
        "Rustle project '{}' initialized successfully at {:?}",
        name, project_root
    );

    Ok(())
}

fn with_ext(string: &str) -> String {
    let ext = constants::FILE_EXTENSIONS[0];
    return format!("{string}.{ext}");
}

// Initialize a project and return the path where it was created
pub async fn initialize_project(path: &Path, name: &str) -> Result<PathBuf> {
    if !path.is_absolute() {
        bail!("Path must be absolute");
    }

    if !path.is_dir() {
        bail!("Path must be a folder");
    }

    let mut root_schema = RootSchema {
        imports: vec![with_ext("./requests/hello")],
        env: HashMap::new(),
        requests: HashMap::new(),
        calls: HashMap::new(),
        project: Some(ProjectDefinationSchema {
            name: name.to_string(),
            version: "0.0.1".to_string(),
            description: format!("{name}, A Native doctor project"),
            authors: vec![],
            generator: None,
        }),
        meta: None,
    };

    root_schema.env.insert(
        "baseurl".to_string(),
        EnvironmentVariableSchema::new(
            serde_yaml::Value::String("https://httpbin.org".to_string()),
            vec![(
                "dev".to_string(),
                serde_yaml::Value::String("http://localhost:8080".to_string()),
            )],
        ),
    );

    root_schema
        .calls
        .insert("main".to_string(), vec!["ping".to_string()]);

    let mut hello_schema = RootSchema {
        imports: vec![],
        env: HashMap::new(),
        requests: HashMap::new(),
        project: None,
        calls: HashMap::new(),
        meta: None,
    };

    let mut ping_request_header = HashMap::new();
    ping_request_header.insert("Accept".to_string(), "application/json".to_string());
    hello_schema.requests.insert(
        "ping".to_string(),
        RequestSchema {
            method: "GET".to_string(),
            url: "{{baseurl}}/get".to_string(),
            doc: "# HttpBin Get\n\nPings the server".to_string(),
            config: None,
            headers: Some(ping_request_header),
            query: None,
            body: None,
            script: None,
        },
    );

    // create root file
    let root_path = path.join(&with_ext(name));
    tokio::fs::write(&root_path, serde_yaml::to_string(&root_schema)?).await?;

    // create request filder
    let requests_folder = path.join("requests");
    tokio::fs::create_dir(&requests_folder).await?;

    // create hello file
    let hello_file = requests_folder.join(&with_ext("hello"));
    tokio::fs::write(hello_file, serde_yaml::to_string(&hello_schema)?).await?;

    return Ok(root_path);
}

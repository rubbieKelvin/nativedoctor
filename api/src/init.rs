use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use tracing::info;

/// Initializes a new Rustle project at the given path with the specified name.
/// Creates the necessary folder structure and initial YAML files.
///
/// Arguments:
/// * `path`: The directory where the project folder should be created.
/// * `name`: The name of the new project folder and main project file.
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
    default: "http://localhost:8000"
    dev: "http://dev.api.example.com"
    prod: "http://prod.api.example.com"
"#;

    let request_file_content = r#"# Example Request Definitions

requests:
  # A simple example GET request
  Ping:
    method: GET
    url: "{{base_url}}/ping" # Uses the base_url from env.api.yaml
    headers:
      Accept: application/json
    # Optional: Add a post-execution script
    # script:
    #   post_request: |
    #     assert response.status === 200;
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

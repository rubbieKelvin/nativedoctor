use std::fs;
use std::fs::File;
use std::io::Write;

use anyhow::Result;
use tempfile::tempdir;

use crate::schema::root::RootSchema;

#[test]
fn test_new_root_schema() -> Result<()> {
    // Create a temporary directory
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create a dummy test.yaml file
    let file_path = dir_path.join("test.yaml");
    let mut file = File::create(&file_path)?;
    writeln!(
        file,
        r#"
imports:
  - ./another.yaml

env:
  API_KEY:
    default: "default_key"
    production: "prod_key"

requests:
  get_user:
    method: GET
    url: "https://api.example.com/user"
"#
    )?;

    let schema = RootSchema::new(&file_path, None)?;

    assert_eq!(schema.imports, vec!["./another.yaml"]);
    assert!(schema.env.contains_key("API_KEY"));
    assert!(schema.requests.contains_key("get_user"));

    let working_dir = schema.get_working_dir();
    assert_eq!(fs::canonicalize(working_dir)?, fs::canonicalize(dir_path)?);

    Ok(())
}

#[test]
fn test_get_all_requests() -> Result<()> {
    // Create a temporary directory
    let dir = tempdir()?;
    let dir_path = dir.path();

    // Create the main test file
    let main_file_path = dir_path.join("main.yaml");
    let mut main_file = File::create(&main_file_path)?;
    writeln!(
        main_file,
        r#"
imports:
  - ./imported.yaml
requests:
  main_request:
    method: GET
    url: "https://main.example.com"
"#
    )?;

    // Create the imported file
    let imported_file_path = dir_path.join("imported.yaml");
    let mut imported_file = File::create(&imported_file_path)?;
    writeln!(
        imported_file,
        r#"
requests:
  imported_request:
    method: POST
    url: "https://imported.example.com"
"#
    )?;

    let schema = RootSchema::new(&main_file_path, None)?;
    let all_requests = schema.get_all_requests()?;

    assert_eq!(all_requests.len(), 2);

    // Check for main_request
    assert!(all_requests
        .iter()
        .any(|(_, name, _)| name == "main_request"));

    // Check for imported_request
    assert!(all_requests
        .iter()
        .any(|(_, name, _)| name == "imported_request"));

    Ok(())
} 
use crate::schema::env::EnvironmentVariableSchema;
use crate::schema::project::ProjectDefinationSchema;
use crate::schema::request::RequestSchema;
use crate::schema::request_body::{MultipartPartSchema, RequestBodySchema};
use crate::schema::request_config::RequestConfigSchema;
use crate::schema::request_script::{RequestScriptConfigSchema, ScriptSchema};
use crate::schema::user::UserSchema;
use anyhow::Result;


#[test]
fn test_deserialize_env_var_schema() -> Result<()> {
    let yaml_str = r#"
default: "default_value"
production: "prod_value"
development: "dev_value"
"#;
    let schema: EnvironmentVariableSchema = serde_yaml::from_str(yaml_str)?;

    assert_eq!(
        schema.default,
        serde_yaml::Value::String("default_value".to_string())
    );
    assert_eq!(
        schema.overrides.get("production"),
        Some(&serde_yaml::Value::String("prod_value".to_string()))
    );
    assert_eq!(
        schema.overrides.get("development"),
        Some(&serde_yaml::Value::String("dev_value".to_string()))
    );

    Ok(())
}

#[test]
fn test_deserialize_project_schema() -> Result<()> {
    let yaml_str = r#"
name: "Test Project"
version: "1.0.0"
description: "A test project"
authors:
  - name: "Test User"
    email: "test@example.com"
generator: "openapi.yaml"
"#;
    let schema: ProjectDefinationSchema = serde_yaml::from_str(yaml_str)?;

    assert_eq!(schema.name, "Test Project");
    assert_eq!(schema.version, "1.0.0");
    assert_eq!(schema.description, "A test project");
    assert_eq!(
        schema.authors,
        vec![UserSchema {
            name: "Test User".to_string(),
            email: "test@example.com".to_string()
        }]
    );
    assert_eq!(schema.generator, Some("openapi.yaml".to_string()));

    Ok(())
}

#[test]
fn test_deserialize_request_schema() -> Result<()> {
    let yaml_str = r#"
method: "POST"
url: "https://api.example.com/data"
doc: "Submit data"
headers:
  Content-Type: "application/json"
query:
  param1: "value1"
body:
  type: "json"
  content:
    key: "value"
"#;
    let schema: RequestSchema = serde_yaml::from_str(yaml_str)?;

    assert_eq!(schema.method, "POST");
    assert_eq!(schema.url, "https://api.example.com/data");
    assert_eq!(schema.doc, "Submit data");
    assert_eq!(
        schema.headers.unwrap().get("Content-Type"),
        Some(&"application/json".to_string())
    );
    assert_eq!(
        schema.query.unwrap().get("param1"),
        Some(&"value1".to_string())
    );

    if let Some(RequestBodySchema::Json { content }) = schema.body {
        let mut expected_content = serde_yaml::Mapping::new();
        expected_content.insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
        assert_eq!(content, serde_yaml::Value::Mapping(expected_content));
    } else {
        panic!("Incorrect body type");
    }

    Ok(())
}

#[test]
fn test_deserialize_request_body_schema() -> Result<()> {
    // Test JSON body
    let json_yaml = r#"
type: "json"
content:
  key: "value"
"#;
    let json_schema: RequestBodySchema = serde_yaml::from_str(json_yaml)?;
    if let RequestBodySchema::Json { content } = json_schema {
        let mut expected_content = serde_yaml::Mapping::new();
        expected_content.insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::String("value".to_string()),
        );
        assert_eq!(content, serde_yaml::Value::Mapping(expected_content));
    } else {
        panic!("Incorrect body type for JSON");
    }

    // Test GraphQL body
    let graphql_yaml = r#"
type: "graphql"
query: "{ hero { name } }"
variables:
  id: 123
"#;
    let graphql_schema: RequestBodySchema = serde_yaml::from_str(graphql_yaml)?;
    if let RequestBodySchema::Graphql { query, variables } = graphql_schema {
        assert_eq!(query, "{ hero { name } }");
        let mut expected_vars = serde_yaml::Mapping::new();
        expected_vars.insert(
            serde_yaml::Value::String("id".to_string()),
            serde_yaml::Value::Number(123.into()),
        );
        assert_eq!(variables, Some(serde_yaml::Value::Mapping(expected_vars)));
    } else {
        panic!("Incorrect body type for GraphQL");
    }

    // Test Multipart body
    let multipart_yaml = r#"
type: "multipart"
parts:
  - kind: "field"
    name: "field1"
    value: "value1"
  - kind: "file"
    name: "file1"
    path: "/path/to/file"
"#;
    let multipart_schema: RequestBodySchema = serde_yaml::from_str(multipart_yaml)?;
    if let RequestBodySchema::Multipart { parts } = multipart_schema {
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            MultipartPartSchema::Field {
                name: "field1".to_string(),
                value: "value1".to_string()
            }
        );
        assert_eq!(
            parts[1],
            MultipartPartSchema::File {
                name: "file1".to_string(),
                path: "/path/to/file".to_string(),
                mime_type: None
            }
        );
    } else {
        panic!("Incorrect body type for Multipart");
    }

    Ok(())
}

#[test]
fn test_deserialize_request_config_schema() -> Result<()> {
    let yaml_str = r#"
depends_on:
  - "request1"
delay: "1s"
timeout: "30s"
retries: 3
folder: "My Folder"
"#;
    let schema: RequestConfigSchema = serde_yaml::from_str(yaml_str)?;

    assert_eq!(schema.depends_on, vec!["request1"]);
    assert_eq!(schema.delay, Some("1s".to_string()));
    assert_eq!(schema.timeout, Some("30s".to_string()));
    assert_eq!(schema.retries, 3);
    assert_eq!(schema.folder, Some("My Folder".to_string()));

    Ok(())
}

#[test]
fn test_deserialize_request_script_schema() -> Result<()> {
    let yaml_str = r#"
pre_request:
  language: "rhai"
  content: "print(\"pre-request script\");"
post_request:
  language: "rhai"
  content: "print(\"post-request script\");"
"#;
    let schema: RequestScriptConfigSchema = serde_yaml::from_str(yaml_str)?;

    if let Some(ScriptSchema::Rhai { content }) = schema.pre_request {
        assert_eq!(content, "print(\"pre-request script\");");
    } else {
        panic!("Incorrect pre-request script type");
    }

    if let Some(ScriptSchema::Rhai { content }) = schema.post_request {
        assert_eq!(content, "print(\"post-request script\");");
    } else {
        panic!("Incorrect post-request script type");
    }

    Ok(())
}

use tempfile::TempDir;
use crate::{
    schemas::request::{RequestSchema, RequestConfigSchema, Method},
    utils::generate_call_request_sequence,
};

/// Helper function to create a temporary directory and return it along with a RequestSchema
fn create_test_request(
    name: &str,
    dependencies: Vec<&str>,
    temp_dir: &TempDir,
) -> anyhow::Result<RequestSchema> {
    let file_name = format!("{}.nd.yaml", name);
    let file_path = temp_dir.path().join(&file_name);
    
    let config = if dependencies.is_empty() {
        None
    } else {
        Some(RequestConfigSchema {
            require: dependencies.iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        })
    };
    
    let schema = RequestSchema {
        name: name.to_string(),
        method: Method::Get,
        url: "https://httpbin.org/get".to_string(),
        config,
        path: Some(file_path.clone()),
        ..Default::default()
    };
    
    // Save the schema to file so dependencies can be loaded
    schema.clone().save_to_path(&file_path)?;
    Ok(schema)
}

#[test]
fn test_basic_sequence_no_dependencies() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let request = create_test_request("simple", vec![], &temp_dir)?;
    
    let sequence = generate_call_request_sequence(request.clone(), vec![])?;
    
    assert_eq!(sequence.len(), 1);
    assert_eq!(sequence[0].name, "simple");
    Ok(())
}

#[test]
fn test_single_dependency() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create dependency first
    let _dependency = create_test_request("login", vec![], &temp_dir)?;
    
    // Create main request that depends on login
    let main_request = create_test_request("protected", vec!["./login.nd.yaml"], &temp_dir)?;
    
    let sequence = generate_call_request_sequence(main_request, vec![])?;
    
    assert_eq!(sequence.len(), 2);
    assert_eq!(sequence[0].name, "login");  // Dependency should be first
    assert_eq!(sequence[1].name, "protected");  // Main request should be last
    Ok(())
}

#[test]
fn test_multiple_dependencies() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create dependencies
    let _auth = create_test_request("auth", vec![], &temp_dir)?;
    let _setup = create_test_request("setup", vec![], &temp_dir)?;
    
    // Create main request that depends on both
    let main_request = create_test_request(
        "main", 
        vec!["./auth.nd.yaml", "./setup.nd.yaml"], 
        &temp_dir
    )?;
    
    let sequence = generate_call_request_sequence(main_request, vec![])?;
    
    assert_eq!(sequence.len(), 3);
    // Dependencies should come first (order may vary)
    let names: Vec<&str> = sequence.iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"auth"));
    assert!(names.contains(&"setup"));
    assert_eq!(sequence.last().unwrap().name, "main");  // Main should be last
    Ok(())
}

#[test]
fn test_nested_dependencies_chain() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create a chain: step1 -> step2 -> step3
    let _step1 = create_test_request("step1", vec![], &temp_dir)?;
    let _step2 = create_test_request("step2", vec!["./step1.nd.yaml"], &temp_dir)?;
    let step3 = create_test_request("step3", vec!["./step2.nd.yaml"], &temp_dir)?;
    
    let sequence = generate_call_request_sequence(step3, vec![])?;
    
    assert_eq!(sequence.len(), 3);
    assert_eq!(sequence[0].name, "step1");  // First in chain
    assert_eq!(sequence[1].name, "step2");  // Second in chain
    assert_eq!(sequence[2].name, "step3");  // Last in chain
    Ok(())
}

#[test]
fn test_cyclic_dependency_detection() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create requests that depend on each other cyclically
    let _request_a = create_test_request("request_a", vec!["./request_b.nd.yaml"], &temp_dir)?;
    let request_b = create_test_request("request_b", vec!["./request_a.nd.yaml"], &temp_dir)?;
    
    let result = generate_call_request_sequence(request_b, vec![]);
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Cyclic dependency detected"));
    Ok(())
}

#[test]
fn test_missing_path_error() {
    let request = RequestSchema {
        name: "no_path".to_string(),
        method: Method::Get,
        url: "https://httpbin.org/get".to_string(),
        path: None,  // Missing path
        ..Default::default()
    };
    
    let result = generate_call_request_sequence(request, vec![]);
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("missing a path"));
    assert!(error_msg.contains("no_path"));
}

#[test]
fn test_complex_dependency_graph_with_shared_dependencies() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create a shared dependency
    let _shared = create_test_request("shared", vec![], &temp_dir)?;
    
    // Create two requests that both depend on shared
    let _request_a = create_test_request("request_a", vec!["./shared.nd.yaml"], &temp_dir)?;
    let _request_b = create_test_request("request_b", vec!["./shared.nd.yaml"], &temp_dir)?;
    
    // Create main request that depends on both A and B
    let main_request = create_test_request(
        "main", 
        vec!["./request_a.nd.yaml", "./request_b.nd.yaml"], 
        &temp_dir
    )?;
    
    let sequence = generate_call_request_sequence(main_request, vec![])?;
    
    // Should have all requests, but shared might appear multiple times
    assert!(sequence.len() >= 4);  // At least main, request_a, request_b, shared
    
    let names: Vec<&str> = sequence.iter().map(|r| r.name.as_str()).collect();
    assert!(names.contains(&"shared"));
    assert!(names.contains(&"request_a"));
    assert!(names.contains(&"request_b"));
    assert_eq!(sequence.last().unwrap().name, "main");
    
    Ok(())
}

#[test]
fn test_self_dependency_detection() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create a request that depends on itself
    let self_dep = create_test_request("self_dep", vec!["./self_dep.nd.yaml"], &temp_dir)?;
    
    let result = generate_call_request_sequence(self_dep, vec![]);
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Cyclic dependency detected"));
    Ok(())
}

#[test]
fn test_deep_nested_chain() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    
    // Create a deep chain: level1 -> level2 -> level3 -> level4 -> level5
    let _level1 = create_test_request("level1", vec![], &temp_dir)?;
    let _level2 = create_test_request("level2", vec!["./level1.nd.yaml"], &temp_dir)?;
    let _level3 = create_test_request("level3", vec!["./level2.nd.yaml"], &temp_dir)?;
    let _level4 = create_test_request("level4", vec!["./level3.nd.yaml"], &temp_dir)?;
    let level5 = create_test_request("level5", vec!["./level4.nd.yaml"], &temp_dir)?;
    
    let sequence = generate_call_request_sequence(level5, vec![])?;
    
    assert_eq!(sequence.len(), 5);
    for (i, expected_name) in ["level1", "level2", "level3", "level4", "level5"].iter().enumerate() {
        assert_eq!(sequence[i].name, *expected_name);
    }
    
    Ok(())
}

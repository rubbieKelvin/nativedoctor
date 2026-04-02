use nd_core::discover::{list_request_paths, list_rhai_paths, partition_valid_request_paths};

#[test]
fn list_request_paths_returns_sorted_supported_files_only() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("b.yaml"), "request: {}\n").unwrap();
    std::fs::write(dir.path().join("a.json"), "{}").unwrap();
    std::fs::write(dir.path().join("c.yml"), "request: {}\n").unwrap();
    std::fs::write(dir.path().join("ignore.rhai"), "let x = 1;").unwrap();
    std::fs::create_dir(dir.path().join("nested")).unwrap();
    std::fs::write(dir.path().join("nested").join("inside.yaml"), "request: {}\n").unwrap();

    let paths = list_request_paths(dir.path()).unwrap();
    let names: Vec<_> = paths
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert_eq!(names, vec!["a.json", "b.yaml", "c.yml"]);
}

#[test]
fn list_request_paths_returns_empty_for_missing_directory() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("does-not-exist");

    let paths = list_request_paths(&missing).unwrap();

    assert!(paths.is_empty());
}

#[test]
fn list_rhai_paths_returns_sorted_rhai_only_non_recursive() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("b.rhai"), "let x = 1;").unwrap();
    std::fs::write(dir.path().join("a.RHAI"), "let y = 2;").unwrap();
    std::fs::write(dir.path().join("ignore.json"), "{}").unwrap();
    std::fs::create_dir(dir.path().join("nested")).unwrap();
    std::fs::write(dir.path().join("nested").join("in.rhai"), "let z = 3;").unwrap();

    let paths = list_rhai_paths(dir.path()).unwrap();
    let names: Vec<_> = paths
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert_eq!(names, vec!["a.RHAI", "b.rhai"]);
}

#[test]
fn list_rhai_paths_returns_empty_for_missing_directory() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing");

    assert!(list_rhai_paths(&missing).unwrap().is_empty());
}

#[test]
fn partition_valid_request_paths_keeps_valid_and_skips_invalid() {
    let dir = tempfile::tempdir().unwrap();
    let good = dir.path().join("ok.yaml");
    std::fs::write(
        &good,
        r#"request:
  method: GET
  url: https://example.com
"#,
    )
    .unwrap();
    let bad = dir.path().join("bad.json");
    std::fs::write(&bad, "not json").unwrap();

    let paths = vec![bad.clone(), good.clone()];
    let (valid, skipped) = partition_valid_request_paths(&paths);

    assert_eq!(valid, vec![good]);
    assert_eq!(skipped.len(), 1);
    assert_eq!(skipped[0].path, bad);
    assert!(!skipped[0].message.is_empty());
}

#[test]
fn partition_valid_request_paths_empty_input() {
    let (valid, skipped) = partition_valid_request_paths(&[]);
    assert!(valid.is_empty());
    assert!(skipped.is_empty());
}

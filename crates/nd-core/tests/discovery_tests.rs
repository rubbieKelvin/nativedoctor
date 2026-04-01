use nd_core::discover::list_request_paths;

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

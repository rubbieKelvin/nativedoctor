//! Tests for non-recursive [`nd_core::list_request_paths`] and workspace catalog classification.

use nd_core::{
    classify_nativedoctor_file, list_request_paths, list_workspace_catalog, WorkspaceFileKind,
};
use std::fs;
use tempfile::tempdir;

#[test]
fn lists_only_immediate_children() {
    let tmp = tempdir().unwrap();
    fs::write(
        tmp.path().join("root.yaml"),
        "request:\n  method: GET\n  url: u\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("nested")).unwrap();
    fs::write(
        tmp.path().join("nested").join("inner.json"),
        "{\"request\":{\"method\":\"GET\",\"url\":\"u\"}}",
    )
    .unwrap();
    let list = list_request_paths(tmp.path()).unwrap();
    assert_eq!(list.len(), 1);
    assert!(list[0].ends_with("root.yaml"));
}

#[test]
fn sorted_order() {
    let tmp = tempdir().unwrap();
    fs::write(
        tmp.path().join("b.yaml"),
        "request:\n  method: GET\n  url: u\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("a.json"),
        "{\"request\":{\"method\":\"GET\",\"url\":\"u\"}}",
    )
    .unwrap();
    let list = list_request_paths(tmp.path()).unwrap();
    assert_eq!(list.len(), 2);
    assert!(list[0].to_string_lossy().contains("a.json"));
    assert!(list[1].to_string_lossy().contains("b.yaml"));
}

#[test]
fn empty_dir_returns_empty() {
    let tmp = tempdir().unwrap();
    let list = list_request_paths(tmp.path()).unwrap();
    assert!(list.is_empty());
}

#[test]
fn missing_dir_returns_empty() {
    let tmp = tempdir().unwrap();
    let missing = tmp.path().join("nope");
    let list = list_request_paths(&missing).unwrap();
    assert!(list.is_empty());
}

#[test]
fn ignores_non_request_extensions() {
    let tmp = tempdir().unwrap();
    fs::write(tmp.path().join("x.txt"), "x").unwrap();
    fs::write(tmp.path().join("y.md"), "x").unwrap();
    let list = list_request_paths(tmp.path()).unwrap();
    assert!(list.is_empty());
}

#[test]
fn classify_request_vs_sequence() {
    let tmp = tempdir().unwrap();
    let req = tmp.path().join("r.yaml");
    fs::write(
        &req,
        "request:\n  method: GET\n  url: https://example.com\n",
    )
    .unwrap();
    let seq = tmp.path().join("s.yaml");
    fs::write(&seq, "steps:\n  - file: r.yaml\n").unwrap();
    assert_eq!(
        classify_nativedoctor_file(&req).unwrap(),
        WorkspaceFileKind::Request
    );
    assert_eq!(
        classify_nativedoctor_file(&seq).unwrap(),
        WorkspaceFileKind::Sequence
    );
}

#[test]
fn list_workspace_catalog_groups_kinds() {
    let tmp = tempdir().unwrap();
    fs::write(
        tmp.path().join("a.yaml"),
        "request:\n  method: GET\n  url: https://example.com\n",
    )
    .unwrap();
    fs::write(tmp.path().join("b.yaml"), "steps:\n  - file: a.yaml\n").unwrap();
    let cat = list_workspace_catalog(tmp.path()).unwrap();
    assert_eq!(cat.len(), 2);
    let kinds: Vec<_> = cat.iter().map(|(_, k)| *k).collect();
    assert!(kinds.contains(&WorkspaceFileKind::Request));
    assert!(kinds.contains(&WorkspaceFileKind::Sequence));
}

//! Tests for [`nd_core::load_request_file`] and [`nd_core::resolve_post_script`].

use std::path::{Path, PathBuf};

use nd_core::load_request_file;
use nd_core::resolve_post_script;
use tempfile::tempdir;

#[test]
fn resolve_post_script_joins_base() {
    assert_eq!(
        resolve_post_script(Path::new("/foo/bar"), "./x.rhai"),
        PathBuf::from("/foo/bar/x.rhai")
    );
}

#[test]
fn load_yaml_minimal_request() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("r.yaml");
    std::fs::write(
        &p,
        b"version: 1\nrequest:\n  method: GET\n  url: https://example.com\npost_script: ./a.rhai\n",
    )
    .unwrap();
    let (req, base) = load_request_file(&p).unwrap();
    assert_eq!(req.request.method, "GET");
    assert_eq!(req.name, None);
    assert_eq!(req.post_script.as_deref(), Some("./a.rhai"));
    assert_eq!(base, dir.path());
}

#[test]
fn load_yaml_with_name() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("r.yaml");
    std::fs::write(
        &p,
        b"version: 1\nname: Health check\nrequest:\n  method: GET\n  url: https://x.test\n",
    )
    .unwrap();
    let (req, _) = load_request_file(&p).unwrap();
    assert_eq!(req.name.as_deref(), Some("Health check"));
}

#[test]
fn load_json_with_name() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("r.json");
    std::fs::write(
        &p,
        r#"{"version":1,"name":"Ping","request":{"method":"GET","url":"https://x"}}"#,
    )
    .unwrap();
    let (req, _) = load_request_file(&p).unwrap();
    assert_eq!(req.name.as_deref(), Some("Ping"));
}

#[test]
fn unsupported_extension_errors() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("x.txt");
    std::fs::write(&p, b"x").unwrap();
    assert!(load_request_file(&p).is_err());
}

#[test]
fn invalid_yaml_errors() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("bad.yaml");
    std::fs::write(&p, b"not: yaml: [[[\n").unwrap();
    assert!(load_request_file(&p).is_err());
}

#[test]
fn invalid_json_errors() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("bad.json");
    std::fs::write(&p, b"{not json").unwrap();
    assert!(load_request_file(&p).is_err());
}

#[test]
fn load_json_openapi_style_metadata() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("r.json");
    std::fs::write(
        &p,
        r#"{"version":1,"request":{"method":"GET","url":"https://api.example/v1","summary":"List","description":"All widgets","tags":["widgets","read"],"deprecated":true}}"#,
    )
    .unwrap();
    let (req, _) = load_request_file(&p).unwrap();
    assert_eq!(req.request.summary.as_deref(), Some("List"));
    assert_eq!(req.request.description.as_deref(), Some("All widgets"));
    assert_eq!(req.request.tags, vec!["widgets", "read"]);
    assert!(req.request.deprecated);
}

#[test]
fn load_yaml_openapi_style_metadata() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("r.yaml");
    std::fs::write(
        &p,
        b"version: 1\nrequest:\n  method: GET\n  url: https://api.example\n  summary: Ping\n  tags:\n    - health\n  deprecated: false\n",
    )
    .unwrap();
    let (req, _) = load_request_file(&p).unwrap();
    assert_eq!(req.request.summary.as_deref(), Some("Ping"));
    assert_eq!(req.request.tags, vec!["health"]);
    assert!(!req.request.deprecated);
}

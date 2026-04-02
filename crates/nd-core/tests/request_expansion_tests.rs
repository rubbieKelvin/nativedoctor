use std::collections::HashMap;

use nd_core::env::RuntimeEnv;
use nd_core::execute::format::format_prepared_request;
use nd_core::model::request::{
    HttpRequestSpec, RequestBody, RequestBodyKind, RequestBodyStructured, RequestFile,
};

fn headers_to_map(headers: Vec<(String, String)>) -> HashMap<String, String> {
    headers.into_iter().collect()
}

#[test]
fn request_expand_overrides_take_precedence_over_env() {
    let env = RuntimeEnv::new();
    env.set("ID", "from-env");

    let spec = HttpRequestSpec {
        method: "GET".into(),
        url: "https://example.test/items/${ID}".into(),
        summary: None,
        description: None,
        tags: vec![],
        deprecated: false,
        query: HashMap::new(),
        headers: HashMap::new(),
        body: None,
        timeout_secs: None,
        follow_redirects: true,
        verify_tls: true,
    };

    let mut overrides = HashMap::new();
    overrides.insert("ID".into(), "from-override".into());

    let prepared = spec
        .expand_with_overrides(&env, Some(&overrides))
        .unwrap();
    assert_eq!(prepared.url, "https://example.test/items/from-override");

    let prepared_env_only = spec.expand(&env).unwrap();
    assert_eq!(prepared_env_only.url, "https://example.test/items/from-env");
}

#[test]
fn request_expand_applies_templates_and_computed_headers() {
    let env = RuntimeEnv::new();
    env.set("HOST", "api.example.test");
    env.set("TOKEN", "secret-token");
    env.set("NAME", "Kelvin");

    let spec = HttpRequestSpec {
        method: "post".into(),
        url: "https://${HOST}/users".into(),
        summary: None,
        description: None,
        tags: vec![],
        deprecated: false,
        query: HashMap::from([("name".into(), "${NAME}".into())]),
        headers: HashMap::from([
            ("Authorization".into(), "Bearer ${TOKEN}".into()),
            ("Content-Type".into(), "application/custom+json".into()),
        ]),
        body: Some(RequestBody::Structured(RequestBodyStructured {
            body_type: RequestBodyKind::Json,
            content: serde_json::json!({
                "name": "${NAME}",
                "enabled": true
            }),
        })),
        timeout_secs: Some(12),
        follow_redirects: false,
        verify_tls: false,
    };

    let prepared = spec.expand(&env).unwrap();
    let headers = headers_to_map(prepared.headers.clone());

    assert_eq!(prepared.method.as_str(), "POST");
    assert_eq!(prepared.url, "https://api.example.test/users");
    assert_eq!(prepared.query, vec![("name".into(), "Kelvin".into())]);
    assert_eq!(
        String::from_utf8(prepared.body.unwrap()).unwrap(),
        r#"{"enabled":true,"name":"Kelvin"}"#
    );
    assert_eq!(
        headers.get("authorization").map(String::as_str),
        Some("Bearer secret-token")
    );
    assert_eq!(
        headers.get("content-type").map(String::as_str),
        Some("application/custom+json")
    );
    assert!(headers.contains_key("user-agent"));
    assert_eq!(headers.get("accept").map(String::as_str), Some("*/*"));
    assert_eq!(prepared.timeout_secs, 12);
    assert!(!prepared.follow_redirects);
    assert!(!prepared.verify_tls);
}

#[test]
fn request_expand_rejects_invalid_binary_body_base64() {
    let env = RuntimeEnv::new();
    let spec = HttpRequestSpec {
        method: "POST".into(),
        url: "https://example.test/upload".into(),
        summary: None,
        description: None,
        tags: vec![],
        deprecated: false,
        query: HashMap::new(),
        headers: HashMap::new(),
        body: Some(RequestBody::Structured(RequestBodyStructured {
            body_type: RequestBodyKind::Binary,
            content: serde_json::Value::String("%%%not-base64%%%".into()),
        })),
        timeout_secs: None,
        follow_redirects: true,
        verify_tls: true,
    };

    let err = match spec.expand(&env) {
        Ok(_) => panic!("expected invalid base64 body expansion to fail"),
        Err(err) => err.to_string(),
    };
    assert!(err.contains("invalid base64"));
}

#[test]
fn format_prepared_request_prints_request_line_headers_and_body() {
    let env = RuntimeEnv::new();
    let spec = HttpRequestSpec {
        method: "POST".into(),
        url: "https://example.test/items".into(),
        summary: None,
        description: None,
        tags: vec![],
        deprecated: false,
        query: HashMap::from([("q".into(), "1".into())]),
        headers: HashMap::from([("x-test".into(), "yes".into())]),
        body: Some(RequestBody::Text("hello world".into())),
        timeout_secs: None,
        follow_redirects: true,
        verify_tls: true,
    };

    let prepared = spec.expand(&env).unwrap();
    let formatted = format_prepared_request(&prepared).unwrap();

    assert!(formatted.starts_with("POST https://example.test/items?q=1\n"));
    assert!(formatted.contains("x-test: yes\n"));
    assert!(formatted.contains("\nhello world\n"));
}

#[test]
fn request_file_from_file_parses_yaml_and_sets_source_path() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("request.yaml");
    std::fs::write(
        &path,
        r#"version: "0.1.1"
name: Example
request:
  method: GET
  url: https://example.test/health
"#,
    )
    .unwrap();

    let doc = RequestFile::from_file(&path).unwrap();

    assert_eq!(doc.name.as_deref(), Some("Example"));
    assert_eq!(doc.request.method, "GET");
    assert_eq!(doc.request.url, "https://example.test/health");
    assert_eq!(doc._path.as_deref(), Some(path.as_path()));
}

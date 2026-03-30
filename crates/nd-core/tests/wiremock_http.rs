use nd_core::{execute_request_with_env, RunOptions, RuntimeEnv};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn post_json_hits_mock_and_returns_body() {
    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id":"abc"})))
        .mount(&mock)
        .await;

    let tmp = tempdir().unwrap();
    let url = format!("{}/api", mock.uri());
    let yaml = format!(
        r#"version: 1
name: Create item
request:
  method: POST
  url: "{}"
  headers:
    Content-Type: application/json
  body:
    type: json
    content:
      x: 1
"#,
        url
    );
    std::fs::write(tmp.path().join("req.yaml"), yaml).unwrap();

    let runtime = RuntimeEnv::from_process_env();
    let opt = RunOptions::default();

    let res = execute_request_with_env(&tmp.path().join("req.yaml"), &opt, &runtime)
        .await
        .unwrap();

    assert_eq!(res.request_name.as_deref(), Some("Create item"));
    assert_eq!(res.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&res.body).unwrap();
    assert_eq!(body["id"], "abc");
}

#[tokio::test]
async fn dry_run_propagates_request_name() {
    let tmp = tempdir().unwrap();
    std::fs::write(
        tmp.path().join("r.yaml"),
        b"version: 1\nname: Probe\nrequest:\n  method: GET\n  url: https://example.com\n",
    )
    .unwrap();

    let runtime = RuntimeEnv::from_process_env();
    let mut opts = RunOptions::default();
    opts.dry_run = true;

    let res = execute_request_with_env(&tmp.path().join("r.yaml"), &opts, &runtime)
        .await
        .unwrap();
    assert_eq!(res.request_name.as_deref(), Some("Probe"));
    assert_eq!(res.status, 0);
}

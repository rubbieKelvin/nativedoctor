use nd_core::{execute_request_file, RunOptions};
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
request:
  method: POST
  url: "{}"
  headers:
    Content-Type: application/json
  body:
    json:
      x: 1
"#,
        url
    );
    std::fs::write(tmp.path().join("req.yaml"), yaml).unwrap();

    let res = execute_request_file(&tmp.path().join("req.yaml"), RunOptions::default())
        .await
        .unwrap();
    assert_eq!(res.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&res.body).unwrap();
    assert_eq!(body["id"], "abc");
}

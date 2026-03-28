use nd_core::{execute_sequence, RunOptions};
use tempfile::tempdir;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn sequence_shares_runtime_token_to_next_request_url() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"t":"abc"})))
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/use/abc"))
        .respond_with(ResponseTemplate::new(200).set_body_string("second-ok"))
        .mount(&mock)
        .await;

    let tmp = tempdir().unwrap();
    let base = mock.uri();
    std::fs::write(
        tmp.path().join("tok.rhai"),
        r#"let j = response_body_json(); set_runtime("TOKEN", j.t);"#,
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("s1.yaml"),
        format!(
            r#"name: Fetch token
request:
  method: GET
  url: "{}/token"
post_script: ./tok.rhai
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("s2.yaml"),
        format!(
            r#"name: Use token
request:
  method: GET
  url: "{}/use/${{TOKEN}}"
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("seq.yaml"),
        b"name: Token flow\nsteps:\n  - file: s1.yaml\n  - file: s2.yaml\n",
    )
    .unwrap();

    let out = execute_sequence(&tmp.path().join("seq.yaml"), &RunOptions::default())
        .await
        .unwrap();
    assert_eq!(out.sequence_name.as_deref(), Some("Token flow"));
    assert_eq!(out.steps.len(), 2);
    assert_eq!(out.steps[0].result.request_name.as_deref(), Some("Fetch token"));
    assert_eq!(out.steps[0].result.status, 200);
    assert_eq!(out.steps[1].result.request_name.as_deref(), Some("Use token"));
    assert_eq!(out.steps[1].result.status, 200);
    assert_eq!(out.steps[1].result.body, b"second-ok");
}

#[tokio::test]
async fn sequence_stops_on_http_error_when_no_post_script() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/bad"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock)
        .await;

    let tmp = tempdir().unwrap();
    let base = mock.uri();
    std::fs::write(
        tmp.path().join("s1.yaml"),
        format!(
            r#"request:
  method: GET
  url: "{}/bad"
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("s2.yaml"),
        format!(
            r#"request:
  method: GET
  url: "{}/bad"
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("seq.yaml"),
        b"steps:\n  - file: s1.yaml\n  - file: s2.yaml\n",
    )
    .unwrap();

    let err = execute_sequence(&tmp.path().join("seq.yaml"), &RunOptions::default())
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("404") || msg.contains("sequence step HTTP"),
        "{}",
        msg
    );
}

#[tokio::test]
async fn sequence_continues_after_http_error_when_post_script_runs() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/bad"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/good"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&mock)
        .await;

    let tmp = tempdir().unwrap();
    let base = mock.uri();
    std::fs::write(tmp.path().join("noop.rhai"), b"1;").unwrap();
    std::fs::write(
        tmp.path().join("s1.yaml"),
        format!(
            r#"request:
  method: GET
  url: "{}/bad"
post_script: ./noop.rhai
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("s2.yaml"),
        format!(
            r#"request:
  method: GET
  url: "{}/good"
"#,
            base
        ),
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("seq.yaml"),
        b"steps:\n  - file: s1.yaml\n  - file: s2.yaml\n",
    )
    .unwrap();

    let out = execute_sequence(&tmp.path().join("seq.yaml"), &RunOptions::default())
        .await
        .unwrap();
    assert_eq!(out.steps.len(), 2);
    assert_eq!(out.steps[0].result.status, 404);
    assert_eq!(out.steps[1].result.status, 200);
    assert_eq!(out.steps[1].result.body, b"ok");
}

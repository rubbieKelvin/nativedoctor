use std::collections::HashMap;

use nd_core::env::RuntimeEnv;
use nd_core::model::request::{HttpRequestSpec, RequestBody, RequestFile};
use wiremock::matchers::{body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn request_file_execute_sends_request_and_returns_response_details() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/echo"))
        .and(header("content-type", "text/plain; charset=utf-8"))
        .and(body_string("hello from nativedoctor"))
        .respond_with(
            ResponseTemplate::new(201)
                .insert_header("x-request-id", "req-123")
                .set_body_string(r#"{"ok":true}"#),
        )
        .mount(&server)
        .await;

    let document = RequestFile {
        version: "0.1.1".into(),
        name: Some("Echo".into()),
        request: HttpRequestSpec {
            method: "POST".into(),
            url: format!("{}/echo", server.uri()),
            summary: None,
            description: None,
            tags: vec![],
            deprecated: false,
            query: HashMap::new(),
            headers: HashMap::new(),
            body: Some(RequestBody::Text("hello from nativedoctor".into())),
            timeout_secs: Some(5),
            follow_redirects: true,
            verify_tls: true,
        },
        _path: None,
    };

    let result = document.execute(&RuntimeEnv::new()).await.unwrap();

    assert_eq!(result.method.as_str(), "POST");
    assert_eq!(result.request_name.as_deref(), Some("Echo"));
    assert_eq!(result.status, 201);
    assert_eq!(result.final_url, format!("{}/echo", server.uri()));
    assert_eq!(String::from_utf8(result.body).unwrap(), r#"{"ok":true}"#);
    assert!(
        result
            .headers
            .iter()
            .any(|(name, value)| name == "x-request-id" && value == "req-123")
    );
    assert!(result.duration.as_nanos() > 0);
}

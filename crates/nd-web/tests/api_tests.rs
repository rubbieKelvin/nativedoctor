use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use nd_web::{api_router, app_router, AppState};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

fn test_state(roots: Vec<PathBuf>) -> AppState {
    AppState {
        roots: Arc::new(roots),
        no_network_io: true,
        env_files: Arc::new(vec![]),
        persistence_file: None,
        sessions: Arc::new(Mutex::new(HashMap::new())),
    }
}

#[tokio::test]
async fn file_outside_roots_is_forbidden() {
    let allowed = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let secret = other.path().join("s.txt");
    std::fs::write(&secret, "x").unwrap();
    let roots = vec![allowed.path().canonicalize().unwrap()];
    let app = api_router(test_state(roots));
    let path = secret.canonicalize().unwrap();
    let uri = format!("/file?path={}", path.to_string_lossy());
    let res = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn workspace_lists_valid_request_skips_bad_json() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("ok.yaml"),
        "request:\n  method: GET\n  url: https://example.com\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("bad.json"), "not-json").unwrap();
    let roots = vec![dir.path().canonicalize().unwrap()];
    let app = api_router(test_state(roots));
    let res = app
        .oneshot(
            Request::builder()
                .uri("/workspace")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["requests"][0]["entries"].as_array().unwrap().len(), 1);
    assert_eq!(v["skipped_requests"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn app_router_root_serves_embedded_index() {
    let dir = tempfile::tempdir().unwrap();
    let roots = vec![dir.path().canonicalize().unwrap()];
    let app = app_router(test_state(roots));
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let ct = res
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.contains("text/html") || ct.contains("html"),
        "unexpected Content-Type: {ct}"
    );
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let s = String::from_utf8_lossy(&body);
    assert!(
        s.to_lowercase().contains("html"),
        "expected HTML document from embedded index"
    );
}

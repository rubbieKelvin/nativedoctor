use std::path::PathBuf;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use nd_web::{api_router, AppState};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;

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
async fn websocket_run_request_dry_run_streams_events_and_run_complete() {
    let dir = tempfile::tempdir().unwrap();
    let req_path = dir.path().join("req.yaml");
    std::fs::write(
        &req_path,
        "request:\n  method: GET\n  url: https://example.com\n",
    )
    .unwrap();

    let roots = vec![dir.path().canonicalize().unwrap()];
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let app = api_router(test_state(roots));
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    tokio::time::sleep(Duration::from_millis(80)).await;

    let url = format!("ws://127.0.0.1:{}/ws", addr.port());
    let (mut ws, _) = tokio_tungstenite::connect_async(url)
        .await
        .expect("websocket connect");

    let cmd = json!({
        "type": "run_request",
        "source_path": req_path.to_string_lossy(),
        "overrides": {},
        "stream": false,
    });
    ws.send(Message::Text(cmd.to_string().into()))
        .await
        .expect("send command");

    let mut saw_event = false;
    let mut complete_ok = false;

    while let Some(msg) = ws.next().await {
        let msg = msg.expect("ws message");
        let Message::Text(t) = msg else {
            continue;
        };
        let v: serde_json::Value = serde_json::from_str(t.as_str()).expect("json");
        let kind = v["kind"].as_str().unwrap_or("");
        if kind == "SessionStarted" || kind == "RuntimeVariablesInitialized" {
            saw_event = true;
        }
        if kind == "run_complete" {
            complete_ok = v["ok"].as_bool() == Some(true);
            break;
        }
    }

    assert!(saw_event, "expected at least one session event");
    assert!(complete_ok, "expected run_complete ok");
}

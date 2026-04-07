//! WebSocket: one JSON command per connection, stream session events as JSON text frames, then `run_complete`.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use nd_core::model::request::RequestFile;
use nd_core::rhai::{resolver::RhaiScriptRunOptions, run::run_rhai_script};
use nd_core::stream::events::Event;
use nd_core::stream::{MutexSession, Session};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::send::{execution_to_dto, runtime_env_for_state, ExecutionResultDto};
use super::AppState;
use crate::path_sandbox::resolve_allowed_file;

/// First message from the client after connect (one run per connection).
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RunCommand {
    RunRequest {
        source_path: String,
        #[serde(default)]
        document: Option<serde_json::Value>,
        #[serde(default)]
        overrides: HashMap<String, String>,
        #[serde(default)]
        stream: bool,
    },
    RunScript {
        path: String,
    },
}

#[derive(Serialize)]
struct RunComplete {
    kind: &'static str,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<ExecutionResultDto>,
}

pub async fn session_ws(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let first = match socket.recv().await {
        Some(Ok(Message::Text(t))) => t.to_string(),
        Some(Ok(Message::Binary(b))) => match String::from_utf8(b.to_vec()) {
            Ok(s) => s,
            Err(_) => {
                let _ = socket
                    .send(Message::Text(
                        r#"{"kind":"error","message":"invalid UTF-8 command"}"#.into(),
                    ))
                    .await;
                return;
            }
        },
        Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) | None => {
            return;
        }
        Some(Ok(Message::Close(_))) => {
            return;
        }
        Some(Err(_)) => {
            return;
        }
    };

    let cmd: RunCommand = match serde_json::from_str(&first) {
        Ok(c) => c,
        Err(e) => {
            let msg = serde_json::to_string(&json!({
                "kind": "error",
                "message": format!("invalid run command: {e}"),
            }))
            .unwrap_or_else(|_| "{}".into());
            let _ = socket.send(Message::Text(msg.into())).await;
            return;
        }
    };

    let runtime = match runtime_env_for_state(&state) {
        Ok(r) => r,
        Err(e) => {
            let msg = serde_json::to_string(&json!({
                "kind": "error",
                "message": e,
            }))
            .unwrap_or_else(|_| "{}".into());
            let _ = socket.send(Message::Text(msg.into())).await;
            return;
        }
    };

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Event>();
    let tx_holder: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<Event>>>> =
        Arc::new(Mutex::new(Some(tx)));

    let tx_for_cb = tx_holder.clone();

    let session = match Session::new(
        move || Ok(runtime),
        Some(Box::new(move |ev| {
            if let Ok(g) = tx_for_cb.lock() {
                if let Some(t) = g.as_ref() {
                    let _ = t.send(ev);
                }
            }
        })),
    ) {
        Ok(s) => s,
        Err(e) => {
            let msg = serde_json::to_string(&json!({
                "kind": "error",
                "message": e,
            }))
            .unwrap_or_else(|_| "{}".into());
            let _ = socket.send(Message::Text(msg.into())).await;
            return;
        }
    };

    let session_id = session.session_id().to_string();
    let arc_session = Arc::new(Mutex::new(session));

    if let Ok(mut g) = state.sessions.lock() {
        g.insert(session_id, arc_session.clone());
    }

    let run_handle = tokio::spawn(run_command(cmd, state, arc_session.clone(), tx_holder));

    while let Some(ev) = rx.recv().await {
        let payload = match serde_json::to_string(&ev) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if socket.send(Message::Text(payload.into())).await.is_err() {
            break;
        }
    }

    let run_result = run_handle.await;

    let (ok, error, result) = match run_result {
        Ok(Ok(outcome)) => (outcome.ok, outcome.error, outcome.result),
        Ok(Err(e)) => (false, Some(e), None),
        Err(e) => (false, Some(format!("run task failed: {e}")), None),
    };

    let complete = serde_json::to_string(&RunComplete {
        kind: "run_complete",
        ok,
        error,
        result,
    })
    .unwrap_or_else(|_| r#"{"kind":"run_complete","ok":false}"#.into());
    let _ = socket.send(Message::Text(complete.into())).await;
}

struct RunOutcome {
    ok: bool,
    error: Option<String>,
    result: Option<ExecutionResultDto>,
}

async fn run_command(
    cmd: RunCommand,
    state: AppState,
    arc_session: Arc<Mutex<Session>>,
    tx_holder: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<Event>>>>,
) -> Result<RunOutcome, String> {
    match cmd {
        RunCommand::RunRequest {
            source_path,
            document,
            overrides,
            stream,
        } => {
            let source = PathBuf::from(&source_path);
            let allowed =
                resolve_allowed_file(&source, state.roots.as_ref()).map_err(|e| e.to_string())?;

            let mut doc = if let Some(v) = document {
                let mut d: RequestFile = serde_json::from_value(v)
                    .map_err(|e| format!("invalid request document: {e}"))?;
                d._path = Some(allowed.clone());
                d
            } else {
                RequestFile::from_file(&allowed).map_err(|e| e.to_string())?
            };
            doc._path = Some(allowed);

            let overrides_ref = if overrides.is_empty() {
                None
            } else {
                Some(&overrides)
            };

            if state.no_network_io {
                let runtime = runtime_env_for_state(&state)?;
                let prep = doc
                    .request
                    .expand_with_overrides(&runtime, overrides_ref)
                    .map_err(|e| e.to_string())?;

                arc_session.emit(|id, e| Event::HttpRequestStarted {
                    session_id: id,
                    request_name: doc.name.clone(),
                    method: prep.method.to_string(),
                    url: prep.url.clone(),
                    elapsed: e,
                });
                arc_session.emit(|id, e| Event::HttpResponseCompleted {
                    session_id: id,
                    request_name: doc.name.clone(),
                    status: 0,
                    final_url: prep.url.clone(),
                    elapsed: e,
                });

                let _ = arc_session.lock().map(|mut s| s.finish());
                if let Ok(mut g) = tx_holder.lock() {
                    g.take();
                }

                return Ok(RunOutcome {
                    ok: true,
                    error: None,
                    result: Some(ExecutionResultDto {
                        status: 0,
                        duration_ms: 0,
                        final_url: prep.url.clone(),
                        method: prep.method.as_str().to_string(),
                        request_name: doc.name.clone(),
                        headers: prep
                            .headers
                            .iter()
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect(),
                        body_text: None,
                        body_base64: None,
                        body_utf8: true,
                    }),
                });
            }

            let res = doc
                .execute_with_overrides(arc_session.clone(), overrides_ref, stream)
                .await;

            let _ = arc_session.lock().map(|mut session| session.finish());

            if let Ok(mut g) = tx_holder.lock() {
                g.take();
            }

            Ok(match res {
                Ok(exec) => RunOutcome {
                    ok: true,
                    error: None,
                    result: Some(execution_to_dto(&exec)),
                },
                Err(e) => RunOutcome {
                    ok: false,
                    error: Some(e.to_string()),
                    result: None,
                },
            })
        }
        RunCommand::RunScript { path } => {
            let p = PathBuf::from(&path);
            let allowed =
                resolve_allowed_file(&p, state.roots.as_ref()).map_err(|e| e.to_string())?;

            let session = arc_session.clone();
            let no_network_io = state.no_network_io;
            let res = tokio::task::spawn_blocking(move || {
                run_rhai_script(&allowed, session, RhaiScriptRunOptions { no_network_io })
            })
            .await
            .map_err(|e| format!("spawn_blocking: {e}"))?;

            let _ = arc_session.lock().map(|mut s| s.finish());
            if let Ok(mut g) = tx_holder.lock() {
                g.take();
            }

            Ok(match res {
                Ok(()) => RunOutcome {
                    ok: true,
                    error: None,
                    result: None,
                },
                Err(e) => RunOutcome {
                    ok: false,
                    error: Some(e.to_string()),
                    result: None,
                },
            })
        }
    }
}

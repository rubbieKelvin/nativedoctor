//! JSON-friendly view of [`nd_core::stream::events::Event`] for WebSocket clients.

use base64::Engine as _;
use nd_core::stream::events::Event;
use serde_json::{json, Value};

fn duration_ms(d: std::time::Duration) -> u128 {
    d.as_millis()
}

/// Serialize a session event for the wire (`Duration` as `elapsed_ms`, bytes as base64).
pub fn event_to_json(ev: &Event) -> Value {
    match ev {
        Event::SessionStarted { id, elapsed } => json!({
            "kind": "SessionStarted",
            "id": id,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::SessionEnded {
            session_id,
            elapsed,
        } => json!({
            "kind": "SessionEnded",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::HttpRequestStarted {
            session_id,
            request_name,
            method,
            url,
            elapsed,
        } => json!({
            "kind": "HttpRequestStarted",
            "session_id": session_id,
            "request_name": request_name,
            "method": method,
            "url": url,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::HttpResponseStreamStarted {
            session_id,
            id,
            elapsed,
            request_name,
            status,
            final_url,
            content_type,
            content_length,
        } => json!({
            "kind": "HttpResponseStreamStarted",
            "session_id": session_id,
            "id": id,
            "elapsed_ms": duration_ms(*elapsed),
            "request_name": request_name,
            "status": status,
            "final_url": final_url,
            "content_type": content_type,
            "content_length": content_length,
        }),
        Event::HttpResponseStreamChunk {
            session_id,
            id,
            request_name,
            sequence,
            data,
            chunk_len,
            bytes_received,
            progress,
            elapsed,
        } => json!({
            "kind": "HttpResponseStreamChunk",
            "session_id": session_id,
            "id": id,
            "request_name": request_name,
            "sequence": sequence,
            "data_base64": base64::engine::general_purpose::STANDARD.encode(data),
            "chunk_len": chunk_len,
            "bytes_received": bytes_received,
            "progress": progress,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::HttpResponseStreamEnded {
            session_id,
            id,
            request_name,
            total_bytes,
            elapsed,
            expected_total,
            length_matched,
        } => json!({
            "kind": "HttpResponseStreamEnded",
            "session_id": session_id,
            "id": id,
            "request_name": request_name,
            "total_bytes": total_bytes,
            "elapsed_ms": duration_ms(*elapsed),
            "expected_total": expected_total,
            "length_matched": length_matched,
        }),
        Event::HttpResponseCompleted {
            session_id,
            request_name,
            status,
            final_url,
            elapsed,
        } => json!({
            "kind": "HttpResponseCompleted",
            "session_id": session_id,
            "request_name": request_name,
            "status": status,
            "final_url": final_url,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::ScriptStarted {
            session_id,
            elapsed,
            script,
        } => json!({
            "kind": "ScriptStarted",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "script": script,
        }),
        Event::ScriptFinished {
            session_id,
            elapsed,
            script,
            success,
            error,
        } => json!({
            "kind": "ScriptFinished",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "script": script,
            "success": success,
            "error": error,
        }),
        Event::RuntimeVariablesInitialized {
            session_id,
            elapsed,
            entries,
        } => json!({
            "kind": "RuntimeVariablesInitialized",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "entries": entries.iter().map(|(k, v)| json!({"key": k, "value": v})).collect::<Vec<_>>(),
        }),
        Event::RuntimeVariablePushed {
            session_id,
            elapsed,
            key,
            value,
            persisted,
        } => json!({
            "kind": "RuntimeVariablePushed",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "key": key,
            "value": value,
            "persisted": persisted,
        }),
        Event::Log {
            session_id,
            level,
            message,
            script,
            elapsed,
        } => json!({
            "kind": "Log",
            "session_id": session_id,
            "level": level.to_string(),
            "message": message,
            "script": script,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::CheckpointWaiting {
            session_id,
            checkpoint_id,
            script,
            message,
            observe,
            elapsed,
        } => json!({
            "kind": "CheckpointWaiting",
            "session_id": session_id,
            "checkpoint_id": checkpoint_id,
            "script": script,
            "message": message,
            "observe": observe,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::CheckpointResumed {
            session_id,
            elapsed,
            checkpoint_id,
        } => json!({
            "kind": "CheckpointResumed",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "checkpoint_id": checkpoint_id,
        }),
        Event::Error {
            session_id,
            elapsed,
            message,
        } => json!({
            "kind": "Error",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "message": message,
        }),
        Event::AssertCalled {
            session_id,
            passed,
            elapsed,
            message,
        } => json!({
            "kind": "AssertCalled",
            "session_id": session_id,
            "passed": passed,
            "elapsed_ms": duration_ms(*elapsed),
            "message": message,
        }),
        Event::NewStepEncountered {
            session_id,
            name,
            elapsed,
        } => json!({
            "kind": "NewStepEncountered",
            "session_id": session_id,
            "name": name,
            "elapsed_ms": duration_ms(*elapsed),
        }),
        Event::FileLoaded {
            session_id,
            elapsed,
            path,
        } => json!({
            "kind": "FileLoaded",
            "session_id": session_id,
            "elapsed_ms": duration_ms(*elapsed),
            "path": path.display().to_string(),
        }),
    }
}

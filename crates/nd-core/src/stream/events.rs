use crate::rhai::LogLevel;
use serde_json::Value;
use std::{path::PathBuf, time::Duration};

/// Streamed update while a run is in progress
#[derive(Debug, Clone)]
pub enum Event {
    /// A new client run was accepted; correlate later events with `session_id`.
    SessionStarted {
        session_id: String,
        elapsed: Duration,
    },
    /// Normal completion (HTTP and script finished, or the run ended without a fatal error).
    SessionEnded {
        session_id: String,
        elapsed: Duration,
    },
    /// Expanded request is about to be sent (or simulated in dry-run).
    HttpRequestStarted {
        request_name: Option<String>,
        method: String,
        url: String,
        elapsed: Duration,
    },
    /// Status and headers are known; the response body will arrive as [`Event::HttpResponseStreamChunk`] events.
    ///
    /// Use this path when the downstream server uses chunked transfer, SSE, NDJSON, or any body streamed over the wire.
    /// Buffered responses can skip these and go straight to [`Event::HttpResponseCompleted`].
    HttpResponseStreamStarted {
        elapsed: Duration,
        request_name: Option<String>,
        status: u16,
        final_url: String,
        /// `Content-Type` from the response, when present (helps the UI choose decoding).
        content_type: Option<String>,
    },
    /// One chunk of the response body as received from the wire (order matches `sequence`).
    HttpResponseStreamChunk {
        request_name: Option<String>,
        sequence: u64,
        data: Vec<u8>,
        elapsed: Duration,
    },
    /// No more body octets for this response; `total_bytes` is the sum of all chunk payloads.
    HttpResponseStreamEnded {
        request_name: Option<String>,
        total_bytes: u64,
        /// Wall time from first byte (or stream start) through end of body.
        elapsed: Duration,
    },
    /// Response received (or dry-run row: `status == 0`, see [`crate::execute::ExecutionResult`]).
    HttpResponseCompleted {
        request_name: Option<String>,
        status: u16,
        final_url: String,
        elapsed: Duration,
    },
    /// Rhai script evaluation began for this label (usually the script path).
    ScriptStarted { elapsed: Duration, script: String },
    /// Rhai script finished; `error` is set when evaluation failed.
    ScriptFinished {
        elapsed: Duration,
        script: String,
        success: bool,
        error: Option<String>,
    },
    RuntimeVariablesInitialized {
        elapsed: Duration,
        entries: Vec<(String, String)>,
    },
    RuntimeVariablePushed {
        elapsed: Duration,
        key: String,
        value: Value,
    },
    /// One line from `log(level, msg)` (and the same shape as [`crate::rhai::Log`]).
    RuntimeLog {
        level: LogLevel,
        message: String,
        script: String,
        elapsed: Duration,
    },
    /// Rhai `checkpoint(when, message, observe)` with `when == true`: execution is blocked until resumed.
    ///
    /// `observe` is the Rhai map converted to JSON for clients (CLI may print it; web can render or inspect it).
    CheckpointWaiting {
        /// Correlate with the “continue” / unblock action (session-scoped or global, depending on host).
        checkpoint_id: String,
        /// Script label (same convention as [`Event::RuntimeLog::script`]).
        script: String,
        message: String,
        observe: Value,
        elapsed: Duration,
    },
    /// Fired after the checkpoint has been acknowledged and the script is running again.
    CheckpointResumed {
        elapsed: Duration,
        checkpoint_id: String,
    },
    /// Fatal error for the session (network, invalid request, unexpected failure).
    Error { elapsed: Duration, message: String },
    AssertCalled {
        passed: bool,
        elapsed: Duration,
        message: String,
    },
    /// we'd add a new rhai function step(string) that just shoots this events. basically does nothing else
    NewStepEncountered { name: String, elapsed: Duration },
    /// Called when we load a file, request or session
    FileLoaded { elapsed: Duration, path: PathBuf },
}

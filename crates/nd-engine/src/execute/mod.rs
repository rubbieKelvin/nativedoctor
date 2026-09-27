//! HTTP execution: expand templates, build a [`reqwest::Client`], send, then stream/collect the body.

pub mod client;
pub mod format;
pub mod prepare;
pub mod types;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use nanoid::nanoid;
use nd_core::model::ResolvedRequest;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Response;
use tracing::debug;

use crate::error::{Error, Result};
use crate::stream::events::Event;
use crate::stream::{MutexSession, Session};
use client::{build_client, send_request};
use prepare::expand_request_with_overrides;
use types::ExecutionResult;

/// Expand `request` against the session's runtime environment and send it, emitting timeline
/// events on `session`. `request_name` is a caller-supplied label (e.g. the operation's
/// `summary`) used for display only.
pub async fn execute_request(
    request: &ResolvedRequest,
    session: Arc<Mutex<Session>>,
    var_overrides: Option<&HashMap<String, String>>,
    request_name: Option<String>,
    stream: bool,
) -> Result<ExecutionResult> {
    let runtime = session
        .lock()
        .expect("session mutex poisoned")
        .runtime
        .clone();

    let prep = expand_request_with_overrides(&runtime, request, var_overrides)?;
    let client = build_client(request)?;

    let start = Instant::now();

    session.emit(|id, e| Event::HttpRequestStarted {
        session_id: id,
        request_name: request_name.clone(),
        method: prep.method.to_string(),
        url: prep.url.clone(),
        elapsed: e,
    });

    let response = send_request(&client, &prep).await?;

    let status = response.status().as_u16();
    let final_url = response.url().to_string();
    let hdrs = response.headers();
    let content_type = header_content_type(hdrs);
    let content_length = header_content_length(hdrs);

    let mut response_headers: Vec<(String, String)> = Vec::new();
    for (name, value) in hdrs.iter() {
        if let Ok(s) = value.to_str() {
            response_headers.push((name.as_str().to_string(), s.to_string()));
        }
    }

    let body = if stream {
        consume_request_stream(
            &session,
            request_name.clone(),
            status,
            final_url.clone(),
            content_type,
            content_length,
            response,
        )
        .await?
    } else {
        response.bytes().await.map_err(Error::Http)?.to_vec()
    };

    let duration = start.elapsed();

    session.emit(|id, e| Event::HttpResponseCompleted {
        session_id: id,
        request_name: request_name.clone(),
        status,
        final_url: final_url.clone(),
        elapsed: e,
    });

    debug!(
        status,
        final_url = %final_url,
        ?duration,
        body_len = body.len(),
        "HTTP response received"
    );

    Ok(ExecutionResult {
        method: prep.method.clone(),
        request_name,
        status,
        final_url,
        headers: response_headers,
        body,
        streamed: stream,
        duration,
        initiator_script: None,
    })
}

fn header_content_type(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(String::from)
}

fn header_content_length(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
}

async fn consume_request_stream(
    session: &Arc<Mutex<Session>>,
    request_name: Option<String>,
    status: u16,
    final_url: String,
    content_type: Option<String>,
    content_length: Option<u64>,
    mut response: Response,
) -> Result<Vec<u8>> {
    let stream_id = nanoid!();

    session.emit(|id, e| Event::HttpResponseStreamStarted {
        session_id: id,
        id: stream_id.clone(),
        elapsed: e,
        request_name: request_name.clone(),
        status,
        final_url: final_url.clone(),
        content_type,
        content_length,
    });

    let mut body = Vec::new();
    let mut sequence: u64 = 0;
    let mut cumulative: u64 = 0;

    loop {
        let chunk = match response.chunk().await.map_err(Error::Http)? {
            Some(c) => c,
            None => break,
        };

        let data = chunk.to_vec();
        let chunk_len = data.len() as u64;
        body.extend_from_slice(&data);

        cumulative += chunk_len;

        let progress = content_length.and_then(|total| {
            if total == 0 {
                return None;
            }
            Some((cumulative as f32 / total as f32).min(1.0))
        });

        session.emit(|id, e| Event::HttpResponseStreamChunk {
            session_id: id,
            id: stream_id.clone(),
            request_name: request_name.clone(),
            sequence,
            data,
            chunk_len,
            bytes_received: cumulative,
            progress,
            elapsed: e,
        });

        sequence = sequence.saturating_add(1);
    }

    let total_bytes = body.len() as u64;

    let length_matched = match content_length {
        Some(expected) => total_bytes == expected,
        None => true,
    };

    session.emit(|id, e| Event::HttpResponseStreamEnded {
        session_id: id,
        id: stream_id.clone(),
        request_name: request_name.clone(),
        total_bytes,
        elapsed: e,
        expected_total: content_length,
        length_matched,
    });

    return Ok(body);
}

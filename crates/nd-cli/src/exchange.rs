//! Runs one prepared request: consumes engine events, prints them, then evaluates checks.

use std::io::{self, Write};
use std::sync::Arc;

use colored::Colorize;
use nd_core::evaluate::{self, ResponseView};
use nd_core::{PreparedRequest, RequestFile, Variables};
use nd_engine::{CookieJar, Event};
use serde_json::{json, Value};
use tokio::sync::mpsc;

use crate::failure::{Failure, ASSERTION};
use crate::render::{self, Print, BINARY_NOTICE};

const CHANNEL_SIZE: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub print: Print,
    pub pretty: bool,
    pub json: bool,
    pub stream: bool,
    pub fail: bool,
    pub show_titles: bool,
}

/// How the response body reaches the terminal or pipe.
#[derive(PartialEq)]
enum BodyMode {
    Hidden,
    /// Bytes pass through untouched, for pipes and files.
    Raw,
    /// Printed line by line, so secrets can be redacted.
    Lines,
    /// Printed once complete, so it can be formatted.
    Buffered,
}

struct Head {
    status: u16,
    reason: Option<String>,
    version: String,
    headers: Vec<(String, String)>,
}

/// Prints the request without sending it. Secrets are always redacted.
pub fn print_offline(
    name: &str,
    request: &PreparedRequest,
    cookies: Option<&Arc<CookieJar>>,
    vars: &Variables,
    output: &Output,
) -> Result<(), Failure> {
    let redactor = vars.redactor();
    let cookie_line = saved_cookies(request, cookies);
    let mut out = io::stdout().lock();
    if output.json {
        let value = json!({
            "name": name,
            "request": request_json(request, cookie_line.as_deref()),
        });
        writeln!(out, "{}", redactor.redact_json(&value))?;
        return Ok(());
    }
    write_request(&mut out, request, cookie_line.as_deref(), vars, output)?;
    return Ok(());
}

/// Saved cookies for this request, values hidden. An explicit `Cookie` header replaces them.
fn saved_cookies(request: &PreparedRequest, cookies: Option<&Arc<CookieJar>>) -> Option<String> {
    let explicit = request
        .headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("cookie"));
    if explicit {
        return None;
    }
    return cookies.and_then(|jar| jar.masked_header(&request.url));
}

fn write_request(
    out: &mut impl Write,
    request: &PreparedRequest,
    cookie_line: Option<&str>,
    vars: &Variables,
    output: &Output,
) -> Result<(), Failure> {
    let redactor = vars.redactor();
    if output.print.request_head {
        let head = render::request_head(request, cookie_line);
        writeln!(out, "{}", redactor.redact(&head))?;
    }
    if output.print.request_body {
        if let Some(body) = render::request_body(request, output.pretty) {
            writeln!(out, "{}\n", redactor.redact(&body))?;
        }
    }
    return Ok(());
}

/// Sends the request. Returns captured values that should be saved.
pub async fn perform(
    name: &str,
    file: &RequestFile,
    request: PreparedRequest,
    cookies: Option<Arc<CookieJar>>,
    vars: &mut Variables,
    output: &Output,
) -> Result<Vec<(String, String)>, Failure> {
    let has_checks = !file.capture.is_empty() || !file.assertions.is_empty();
    let events_as_json = output.json && output.stream;
    let wants_body = output.print.response_body || has_checks || output.json;
    let cookie_line = saved_cookies(&request, cookies.as_ref());

    let mut out = io::stdout().lock();
    if !output.json {
        write_request(&mut out, &request, cookie_line.as_deref(), vars, output)?;
    }

    let (sender, mut events) = mpsc::channel(CHANNEL_SIZE);
    let engine = tokio::spawn(nd_engine::send(request.clone(), cookies, sender));

    let redactor = vars.redactor();
    let mut head: Option<Head> = None;
    let mut mode = BodyMode::Hidden;
    let mut collected: Vec<u8> = Vec::new();
    let mut collect = output.json || has_checks;
    let mut pending_line: Vec<u8> = Vec::new();
    let mut binary_shown = false;
    let mut elapsed_ms: Option<u128> = None;

    while let Some(event) = events.recv().await {
        match event {
            Event::Head {
                status,
                reason,
                version,
                headers,
            } => {
                if events_as_json {
                    let line = json!({
                        "event": "head",
                        "status": status,
                        "version": version,
                        "headers": render::pairs_json(&headers),
                    });
                    writeln!(out, "{}", redactor.redact_json(&line))?;
                    out.flush()?;
                } else if !output.json && output.print.response_head {
                    let text = render::response_head(&version, status, reason.as_deref(), &headers);
                    writeln!(out, "{}", redactor.redact(&text))?;
                    out.flush()?;
                }

                let is_event_stream = headers.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case("content-type")
                        && value.starts_with("text/event-stream")
                });
                mode = choose_mode(output, is_event_stream);
                collect = collect || mode == BodyMode::Buffered;
                head = Some(Head {
                    status,
                    reason,
                    version,
                    headers,
                });
                if !wants_body {
                    break;
                }
            }
            Event::Chunk(bytes) => {
                if collect {
                    collected.extend_from_slice(&bytes);
                }
                if events_as_json {
                    let text = String::from_utf8_lossy(&bytes);
                    let line = json!({ "event": "chunk", "text": text });
                    writeln!(out, "{}", redactor.redact_json(&line))?;
                    out.flush()?;
                }
                match mode {
                    BodyMode::Raw => {
                        out.write_all(&bytes)?;
                        out.flush()?;
                    }
                    BodyMode::Lines if !binary_shown => {
                        if bytes.contains(&0) {
                            writeln!(out, "{BINARY_NOTICE}")?;
                            binary_shown = true;
                            pending_line.clear();
                            continue;
                        }
                        pending_line.extend_from_slice(&bytes);
                        write_complete_lines(&mut out, &mut pending_line, vars)?;
                    }
                    _ => {}
                }
            }
            Event::Finished { elapsed } => {
                elapsed_ms = Some(elapsed.as_millis());
                if events_as_json {
                    writeln!(
                        out,
                        "{}",
                        json!({ "event": "finished", "elapsed_ms": elapsed_ms })
                    )?;
                    out.flush()?;
                }
            }
        }
    }
    drop(events);

    engine
        .await
        .map_err(|e| Failure::new(format!("request task failed: {e}")))??;

    let Some(head) = head else {
        return Err(Failure::new("no response received"));
    };

    let view = ResponseView {
        status: head.status,
        headers: &head.headers,
        body: &collected,
    };
    let failures = evaluate::check(&file.assertions, &view, vars)?;
    let captures = if failures.is_empty() {
        evaluate::capture(&file.capture, &view)
    } else {
        Ok(Vec::new())
    };

    // Captured secrets are stored first, so they're redacted in the output below.
    let mut saved = Vec::new();
    if let Ok(values) = &captures {
        for (name, value) in values {
            if vars.capture(name, value.clone()) {
                saved.push((name.clone(), value.clone()));
            } else {
                eprintln!("warning: capture `{name}` ignored; a value set by --var or the environment wins");
            }
        }
    }
    let redactor = vars.redactor();

    if !pending_line.is_empty() {
        writeln!(
            out,
            "{}",
            redactor.redact(&String::from_utf8_lossy(&pending_line))
        )?;
    }
    if mode == BodyMode::Buffered && !collected.is_empty() {
        let text = render::body_text(&collected, output.pretty);
        writeln!(out, "{}", redactor.redact(&text))?;
    }
    if output.print.meta && !output.json {
        let elapsed = elapsed_ms.unwrap_or_default() as f64 / 1000.0;
        writeln!(out, "\n{} {elapsed:.3}s", "Elapsed time:".dimmed())?;
    }
    if output.json && !output.stream {
        let sent = request_json(&request, cookie_line.as_deref());
        let result = result_json(name, sent, &head, &collected, elapsed_ms, &failures);
        writeln!(out, "{}", redactor.redact_json(&result))?;
    }
    out.flush()?;

    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("{} {}", "assertion failed:".red(), redactor.redact(failure));
        }
        return Err(Failure {
            code: ASSERTION,
            message: String::new(),
        });
    }
    captures?;

    if output.fail && (300..600).contains(&head.status) {
        return Err(Failure {
            code: (head.status / 100) as u8,
            message: format!("server responded with {}", head.status),
        });
    }
    return Ok(saved);
}

fn choose_mode(output: &Output, is_event_stream: bool) -> BodyMode {
    if output.json || !output.print.response_body {
        return BodyMode::Hidden;
    }
    if !output.pretty {
        return BodyMode::Raw;
    }
    if output.stream || is_event_stream {
        return BodyMode::Lines;
    }
    return BodyMode::Buffered;
}

fn write_complete_lines(
    out: &mut impl Write,
    pending: &mut Vec<u8>,
    vars: &Variables,
) -> io::Result<()> {
    let Some(end) = pending.iter().rposition(|byte| *byte == b'\n') else {
        return Ok(());
    };
    let complete: Vec<u8> = pending.drain(..=end).collect();
    let text = vars.redactor().redact(&String::from_utf8_lossy(&complete));
    out.write_all(text.as_bytes())?;
    return out.flush();
}

fn request_json(request: &PreparedRequest, cookie_line: Option<&str>) -> Value {
    let mut headers = request.headers.clone();
    if let Some(cookies) = cookie_line {
        headers.push(("cookie".to_string(), cookies.to_string()));
    }
    return json!({
        "method": request.method,
        "url": request.url.as_str(),
        "headers": render::pairs_json(&headers),
    });
}

fn result_json(
    name: &str,
    request: Value,
    head: &Head,
    body: &[u8],
    elapsed_ms: Option<u128>,
    failures: &[String],
) -> Value {
    return json!({
        "name": name,
        "request": request,
        "response": {
            "status": head.status,
            "reason": head.reason,
            "version": head.version,
            "headers": render::pairs_json(&head.headers),
            "body": render::body_json(body),
            "elapsed_ms": elapsed_ms,
        },
        "assertion_failures": failures,
    });
}

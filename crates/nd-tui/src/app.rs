//! TUI application state: event aggregation, scrolling, and key handling.

use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use nd_core::stream::events::Event;
use serde_json::Value;

use crate::runner::TuiMsg;

const BODY_CAP: usize = 512 * 1024;
const TIMELINE_CAP: usize = 400;
const LOG_CAP: usize = 500;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum PaneFocus {
    #[default]
    Timeline,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum RightTab {
    #[default]
    Preview,
    Logs,
    Body,
    Vars,
}

pub struct App {
    pub timeline: Vec<String>,
    pub timeline_scroll: usize,
    pub logs: Vec<(String, String)>,
    pub log_scroll: usize,
    pub request_preview: String,
    pub body_text: String,
    pub body_scroll: usize,
    pub vars_lines: Vec<String>,
    pub vars_scroll: usize,
    pub stream_label: String,
    pub stream_ratio: Option<f64>,
    pub current_path: String,
    pub session_label: String,
    pub checkpoint_note: String,
    pub footer_status: String,
    pub runner_done: bool,
    pub runner_error: Option<String>,
    pub show_hex_body: bool,
    pub focus: PaneFocus,
    pub right_tab: RightTab,
    body_raw: Vec<u8>,
}

impl App {
    pub fn new() -> Self {
        Self {
            timeline: Vec::new(),
            timeline_scroll: 0,
            logs: Vec::new(),
            log_scroll: 0,
            request_preview: String::new(),
            body_text: String::new(),
            body_scroll: 0,
            vars_lines: Vec::new(),
            vars_scroll: 0,
            stream_label: String::new(),
            stream_ratio: None,
            current_path: String::new(),
            session_label: String::new(),
            checkpoint_note: String::new(),
            footer_status: String::from("running…"),
            runner_done: false,
            runner_error: None,
            show_hex_body: false,
            focus: PaneFocus::Timeline,
            right_tab: RightTab::Preview,
            body_raw: Vec::new(),
        }
    }

    pub fn should_fail_exit(&self) -> bool {
        self.runner_error.is_some()
    }

    pub fn apply(&mut self, msg: TuiMsg) {
        match msg {
            TuiMsg::Event(ev) => self.apply_event(ev),
            TuiMsg::RequestSummary(s) => {
                self.request_preview = s;
                self.push_timeline("request expanded (preview updated)".into());
            }
            TuiMsg::BufferedHttpBody {
                status,
                final_url,
                body,
            } => {
                self.body_raw = body;
                self.refresh_body_display();
                self.push_timeline(format!("response body buffered: HTTP {status} {final_url}"));
            }
            TuiMsg::RunnerFinished(result) => {
                self.runner_done = true;
                match result {
                    Ok(()) => {
                        self.footer_status = "finished — q to quit".into();
                    }
                    Err(e) => {
                        self.runner_error = Some(e.clone());
                        self.footer_status = format!("failed: {e}");
                    }
                }
            }
        }
    }

    fn push_timeline(&mut self, line: String) {
        if self.timeline.len() >= TIMELINE_CAP {
            let drain = self.timeline.len() - TIMELINE_CAP + 50;
            self.timeline.drain(..drain);
            if self.timeline_scroll > 0 {
                self.timeline_scroll = self.timeline_scroll.saturating_sub(drain);
            }
        }
        self.timeline.push(line);
    }

    fn push_body_chunk(&mut self, data: &[u8]) {
        self.body_raw.extend_from_slice(data);
        if self.body_raw.len() > BODY_CAP {
            let excess = self.body_raw.len() - BODY_CAP;
            self.body_raw.drain(..excess);
        }
        self.refresh_body_display();
    }

    fn refresh_body_display(&mut self) {
        if self.show_hex_body {
            let mut s = String::new();
            for (i, chunk) in self.body_raw.chunks(16).enumerate() {
                if i > 0 {
                    s.push('\n');
                }
                s.push_str(&format!("{i:06x}: "));
                for b in chunk {
                    s.push_str(&format!("{b:02x} "));
                }
            }
            self.body_text = s;
        } else {
            self.body_text = String::from_utf8_lossy(&self.body_raw).into_owned();
        }
    }

    fn apply_event(&mut self, ev: Event) {
        let line = match ev {
            Event::SessionStarted { id, elapsed } => {
                self.session_label = id.chars().take(8).collect();
                format!("[{:>6.3}s] session started ({})", fmt_secs(elapsed), id)
            }
            Event::SessionEnded { elapsed, .. } => {
                self.stream_ratio = None;
                self.stream_label.clear();
                format!("[{:>6.3}s] session ended", fmt_secs(elapsed))
            }
            Event::FileLoaded { path, elapsed, .. } => {
                self.current_path = path.display().to_string();
                format!(
                    "[{:>6.3}s] file loaded: {}",
                    fmt_secs(elapsed),
                    path.display()
                )
            }
            Event::HttpRequestStarted {
                method,
                url,
                request_name,
                elapsed,
                ..
            } => {
                let name = request_name.unwrap_or_default();
                format!(
                    "[{:>6.3}s] HTTP → {method} {name} {url}",
                    fmt_secs(elapsed)
                )
            }
            Event::HttpResponseStreamStarted {
                status,
                final_url,
                content_length,
                request_name,
                elapsed,
                ..
            } => {
                self.stream_ratio = Some(0.0);
                let name = request_name.unwrap_or_default();
                let cl = content_length
                    .map(|n| format!("{n} bytes"))
                    .unwrap_or_else(|| "unknown length".into());
                self.stream_label = format!("streaming HTTP {status} {name} ({cl})");
                format!(
                    "[{:>6.3}s] response stream started: {status} {final_url}",
                    fmt_secs(elapsed)
                )
            }
            Event::HttpResponseStreamChunk {
                data,
                bytes_received,
                progress,
                ..
            } => {
                self.push_body_chunk(&data);
                if let Some(p) = progress {
                    self.stream_ratio = Some(p as f64);
                }
                self.stream_label = format!(
                    "receiving… {} bytes{}",
                    bytes_received,
                    progress
                        .map(|p| format!(" ({:.0}%)", p * 100.0))
                        .unwrap_or_default()
                );
                return;
            }
            Event::HttpResponseStreamEnded {
                total_bytes,
                length_matched,
                ..
            } => {
                self.stream_ratio = None;
                self.stream_label.clear();
                format!(
                    "stream ended: {} bytes (length ok: {length_matched})",
                    total_bytes
                )
            }
            Event::HttpResponseCompleted {
                status,
                final_url,
                request_name,
                elapsed,
                ..
            } => {
                let name = request_name.unwrap_or_default();
                format!(
                    "[{:>6.3}s] HTTP done {status} {name} {final_url}",
                    fmt_secs(elapsed)
                )
            }
            Event::ScriptStarted { script, elapsed, .. } => {
                format!("[{:>6.3}s] script started: {script}", fmt_secs(elapsed))
            }
            Event::ScriptFinished {
                script,
                success,
                error,
                elapsed,
                ..
            } => {
                let err = error
                    .map(|e| format!(" — {e}"))
                    .unwrap_or_default();
                format!(
                    "[{:>6.3}s] script {}finished: {script}{err}",
                    fmt_secs(elapsed),
                    if success { "" } else { "not " }
                )
            }
            Event::RuntimeVariablesInitialized { entries, elapsed, .. } => {
                self.vars_lines = entries
                    .iter()
                    .map(|(k, v)| format!("{k} = {v}"))
                    .collect();
                format!(
                    "[{:>6.3}s] runtime vars initialized ({} entries)",
                    fmt_secs(elapsed),
                    self.vars_lines.len()
                )
            }
            Event::RuntimeVariablePushed {
                key,
                value,
                persisted,
                elapsed,
                ..
            } => {
                let v = value_to_compact(&value);
                self.vars_lines.push(format!(
                    "{key} = {v}{}",
                    if persisted { " (persisted)" } else { "" }
                ));
                if self.vars_lines.len() > 200 {
                    self.vars_lines.drain(0..50);
                }
                format!("[{:>6.3}s] var push: {key}", fmt_secs(elapsed))
            }
            Event::Log {
                level,
                message,
                script,
                ..
            } => {
                let lvl = level.as_str();
                self.logs
                    .push((lvl.to_string(), format!("[{script}] {message}")));
                if self.logs.len() > LOG_CAP {
                    self.logs.drain(0..50);
                }
                format!("[log:{lvl}] {message}")
            }
            Event::CheckpointWaiting {
                message,
                script,
                observe,
                ..
            } => {
                let obs = serde_json::to_string(&observe).unwrap_or_else(|_| "{}".into());
                self.checkpoint_note = format!("checkpoint ({script}): {message}\n{obs}");
                format!("checkpoint waiting: {message}")
            }
            Event::CheckpointResumed { checkpoint_id, .. } => {
                self.checkpoint_note.clear();
                format!("checkpoint resumed: {checkpoint_id}")
            }
            Event::Error { message, .. } => {
                format!("ERROR: {message}")
            }
            Event::AssertCalled {
                passed, message, ..
            } => {
                format!(
                    "assert {}: {message}",
                    if passed { "ok" } else { "FAILED" }
                )
            }
            Event::NewStepEncountered { name, .. } => {
                format!("step: {name}")
            }
        };
        self.push_timeline(line);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.kind != crossterm::event::KeyEventKind::Press {
            return false;
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            return true;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return true,
            KeyCode::Tab => {
                self.focus = match self.focus {
                    PaneFocus::Timeline => PaneFocus::Right,
                    PaneFocus::Right => PaneFocus::Timeline,
                };
            }
            KeyCode::Char('1') => self.right_tab = RightTab::Preview,
            KeyCode::Char('2') => self.right_tab = RightTab::Logs,
            KeyCode::Char('3') => self.right_tab = RightTab::Body,
            KeyCode::Char('4') => self.right_tab = RightTab::Vars,
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.show_hex_body = !self.show_hex_body;
                self.refresh_body_display();
            }
            KeyCode::Up | KeyCode::Char('k') => self.scroll_active(-1),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_active(1),
            KeyCode::PageUp => self.scroll_active(-10),
            KeyCode::PageDown => self.scroll_active(10),
            _ => {}
        }
        false
    }

    fn scroll_active(&mut self, delta: i32) {
        let d = delta as isize;
        match self.focus {
            PaneFocus::Timeline => {
                self.timeline_scroll = (self.timeline_scroll as isize + d).max(0) as usize;
            }
            PaneFocus::Right => match self.right_tab {
                RightTab::Preview => {}
                RightTab::Logs => {
                    self.log_scroll = (self.log_scroll as isize + d).max(0) as usize;
                }
                RightTab::Body => {
                    self.body_scroll = (self.body_scroll as isize + d).max(0) as usize;
                }
                RightTab::Vars => {
                    self.vars_scroll = (self.vars_scroll as isize + d).max(0) as usize;
                }
            },
        }
    }
}

fn fmt_secs(d: Duration) -> f64 {
    d.as_secs_f64()
}

fn value_to_compact(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => v.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nd_core::rhai::logger::LogLevel;

    #[test]
    fn log_event_appends_logs_and_timeline() {
        let mut app = App::new();
        app.apply(TuiMsg::Event(Event::Log {
            session_id: "x".into(),
            level: LogLevel::Info,
            message: "hello".into(),
            script: "s.rhai".into(),
            elapsed: Duration::from_millis(1),
        }));
        assert_eq!(app.logs.len(), 1);
        assert!(app.timeline.last().unwrap().contains("hello"));
    }

    #[test]
    fn runner_finished_sets_done() {
        let mut app = App::new();
        app.apply(TuiMsg::RunnerFinished(Ok(())));
        assert!(app.runner_done);
        assert!(app.runner_error.is_none());
    }
}

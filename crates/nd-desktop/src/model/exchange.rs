//! What the response panel shows, built up from a run's updates.

use nd_core::Redactor;
use nd_engine::report::{body_text, BINARY_NOTICE};
use nd_engine::ResponseHead;

use super::run::{Outcome, RunResult, SentRequest, Update};

/// Larger bodies are cut for display. The full bytes stay available for checks and Save.
pub const DISPLAY_LIMIT: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    Json,
    Html,
    Plain,
}

#[derive(Debug, Default)]
pub struct Exchange {
    pub running: bool,
    pub name: String,
    pub sent: Option<SentRequest>,
    pub head: Option<ResponseHead>,
    /// Shown as it arrives, for streams.
    pub live: bool,
    pub live_body: Vec<u8>,
    pub result: Option<RunResult>,
    redactor: Redactor,
}

impl Exchange {
    pub fn start() -> Exchange {
        return Exchange {
            running: true,
            ..Exchange::default()
        };
    }

    /// Returns true once the run is over.
    pub fn apply(&mut self, update: Update) -> bool {
        match update {
            Update::Started { name, .. } => self.name = name,
            Update::Prepared {
                sent, redactor, ..
            } => {
                self.sent = Some(sent);
                self.redactor = redactor;
            }
            Update::Head { head, live, .. } => {
                self.head = Some(head);
                self.live = live;
            }
            Update::Chunk { bytes, .. } => {
                if self.live_body.len() < DISPLAY_LIMIT {
                    self.live_body.extend_from_slice(&bytes);
                }
            }
            Update::Done { result, .. } => {
                self.redactor = result.redactor.clone();
                self.result = Some(*result);
            }
            Update::Finished => {
                self.running = false;
                return true;
            }
        }
        return false;
    }

    /// Stops showing a run that was cancelled.
    pub fn cancelled(&mut self) {
        self.running = false;
    }

    pub fn status_text(&self) -> String {
        if let Some(Outcome::Error { message, .. }) = self.outcome() {
            return message.clone();
        }
        if self.running && self.head.is_none() {
            return "Sending…".to_string();
        }
        let Some(head) = &self.head else {
            return String::new();
        };
        let reason = head.reason.as_deref().unwrap_or("");
        let mut text = format!("{} {reason}", head.status).trim_end().to_string();
        if let Some(elapsed) = self.result.as_ref().and_then(|r| r.elapsed) {
            text.push_str(&format!(" · {} ms", elapsed.as_millis()));
        }
        let size = self.body_len();
        if size > 0 {
            text.push_str(&format!(" · {}", human_size(size)));
        }
        return text;
    }

    pub fn outcome(&self) -> Option<&Outcome> {
        return self.result.as_ref().map(|result| &result.outcome);
    }

    pub fn body_len(&self) -> usize {
        return match &self.result {
            Some(result) => result.body.len(),
            None => self.live_body.len(),
        };
    }

    pub fn content_type(&self) -> Option<&str> {
        let head = self.head.as_ref()?;
        return head
            .headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-type"))
            .map(|(_, value)| value.as_str());
    }

    pub fn language(&self) -> Language {
        let kind = self.content_type().unwrap_or("").to_ascii_lowercase();
        if kind.contains("json") {
            return Language::Json;
        }
        if kind.contains("html") {
            return Language::Html;
        }
        return Language::Plain;
    }

    /// The body as text, with secrets hidden. `pretty` formats JSON.
    pub fn body_display(&self, pretty: bool) -> String {
        let bytes: &[u8] = match &self.result {
            Some(result) => &result.body,
            None => &self.live_body,
        };
        if bytes.is_empty() {
            return String::new();
        }
        let shown = &bytes[..bytes.len().min(DISPLAY_LIMIT)];
        let mut text = if self.result.is_some() {
            body_text(shown, pretty)
        } else {
            self.live_text(shown)
        };
        text = self.redactor.redact(&text);
        if bytes.len() > DISPLAY_LIMIT {
            text.push_str(&format!(
                "\n\n[showing the first {} of {}; Save has the whole body]",
                human_size(DISPLAY_LIMIT),
                human_size(bytes.len())
            ));
        }
        return text;
    }

    /// Complete lines only, so a secret split across chunks can't show partially.
    fn live_text(&self, bytes: &[u8]) -> String {
        if bytes.contains(&0) {
            return BINARY_NOTICE.to_string();
        }
        let end = bytes.iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
        return String::from_utf8_lossy(&bytes[..end]).to_string();
    }

    pub fn headers_display(&self) -> String {
        let Some(head) = &self.head else {
            return String::new();
        };
        let lines: Vec<String> = head
            .headers
            .iter()
            .map(|(name, value)| format!("{name}: {value}"))
            .collect();
        return self.redactor.redact(&lines.join("\n"));
    }

    pub fn sent_display(&self) -> String {
        let Some(sent) = &self.sent else {
            return String::new();
        };
        return match &sent.body {
            Some(body) => format!("{}\n\n{body}", sent.head),
            None => sent.head.clone(),
        };
    }

    pub fn failures(&self) -> &[String] {
        return match self.outcome() {
            Some(Outcome::Assertions(failures)) => failures,
            _ => &[],
        };
    }
}

pub fn human_size(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    let size = bytes as f64;
    if size < KIB {
        return format!("{bytes} B");
    }
    if size < KIB * KIB {
        return format!("{:.1} KB", size / KIB);
    }
    return format!("{:.1} MB", size / (KIB * KIB));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;
    use crate::model::run::RunResult;

    fn head(status: u16, content_type: &str) -> ResponseHead {
        return ResponseHead {
            status,
            reason: Some("OK".to_string()),
            version: "HTTP/1.1".to_string(),
            headers: vec![("content-type".to_string(), content_type.to_string())],
        };
    }

    fn result(body: &[u8], redactor: Redactor) -> RunResult {
        return RunResult {
            outcome: Outcome::Passed,
            body: Arc::new(body.to_vec()),
            elapsed: Some(Duration::from_millis(84)),
            redactor,
            captured: Vec::new(),
            warnings: Vec::new(),
            json: None,
        };
    }

    #[test]
    fn a_finished_exchange_summarises_itself() {
        let mut exchange = Exchange::start();
        assert_eq!(exchange.status_text(), "Sending…");
        exchange.apply(Update::Head {
            index: 0,
            head: head(200, "application/json"),
            live: false,
        });
        exchange.apply(Update::Done {
            index: 0,
            result: Box::new(result(b"{\"a\":1}", Redactor::default())),
        });
        assert!(exchange.apply(Update::Finished));
        assert!(!exchange.running);
        assert_eq!(exchange.status_text(), "200 OK · 84 ms · 7 B");
        assert_eq!(exchange.language(), Language::Json);
        assert_eq!(exchange.body_display(true), "{\n  \"a\": 1\n}");
        assert_eq!(exchange.body_display(false), "{\"a\":1}");
    }

    #[test]
    fn errors_replace_the_status() {
        let mut exchange = Exchange::start();
        let mut failed = result(b"", Redactor::default());
        failed.outcome = Outcome::Error {
            kind: crate::model::run::FailureKind::Timeout,
            message: "request timed out after 30s".to_string(),
        };
        exchange.apply(Update::Done {
            index: 0,
            result: Box::new(failed),
        });
        assert_eq!(exchange.status_text(), "request timed out after 30s");
    }

    #[test]
    fn live_bodies_show_complete_lines_only() {
        let mut exchange = Exchange::start();
        exchange.apply(Update::Head {
            index: 0,
            head: head(200, "text/event-stream"),
            live: true,
        });
        exchange.apply(Update::Chunk {
            index: 0,
            bytes: b"data: one\ndata: tw".to_vec(),
        });
        assert_eq!(exchange.body_display(true), "data: one\n");
    }

    #[test]
    fn secrets_are_hidden_in_every_view() {
        let mut exchange = Exchange::start();
        let redactor = redactor_for("hunter2");
        exchange.apply(Update::Head {
            index: 0,
            head: ResponseHead {
                headers: vec![("x-echo".to_string(), "hunter2".to_string())],
                ..head(200, "text/plain")
            },
            live: false,
        });
        exchange.apply(Update::Done {
            index: 0,
            result: Box::new(result(b"token hunter2", redactor)),
        });
        assert_eq!(exchange.body_display(true), "token ***");
        assert_eq!(exchange.headers_display(), "x-echo: ***");
    }

    /// A redactor that hides `secret`, built the way real ones are: from a declared secret.
    fn redactor_for(secret: &str) -> Redactor {
        let dir = tempfile::tempdir().unwrap();
        let collection = nd_core::Collection::init(dir.path(), None).unwrap();
        std::fs::write(
            dir.path().join("environments/dev.yaml"),
            "vars: {}\nsecrets: [S]\n",
        )
        .unwrap();
        let path = collection.secrets_path("dev");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, format!("S={secret}\n")).unwrap();
        let vars =
            nd_core::Variables::load(Some(&collection), Some("dev"), &[], &|_| None).unwrap();
        return vars.redactor();
    }

    #[test]
    fn long_bodies_are_cut_for_display_only() {
        let mut exchange = Exchange::start();
        let big = vec![b'a'; DISPLAY_LIMIT + 10];
        exchange.apply(Update::Done {
            index: 0,
            result: Box::new(result(&big, Redactor::default())),
        });
        let shown = exchange.body_display(false);
        assert!(shown.contains("showing the first 5.0 MB"));
        assert_eq!(exchange.body_len(), DISPLAY_LIMIT + 10);
    }

    #[test]
    fn sizes_read_naturally() {
        assert_eq!(human_size(12), "12 B");
        assert_eq!(human_size(2048), "2.0 KB");
        assert_eq!(human_size(3 * 1024 * 1024), "3.0 MB");
    }
}

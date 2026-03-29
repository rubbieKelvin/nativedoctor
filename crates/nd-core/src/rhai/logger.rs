//! In-memory log sink for Rhai post-scripts ([`Logger::log`] via the `log(level, message)` host fn).
//!
//! Entries are ordered; [`Log::elapsed`] is measured from when the [`Logger`] was created (useful for
//! timing relative to script start). Use [`Logger::snapshot`] or [`Logger::drain`] after
//! [`crate::rhai::host::run_post_script`] when a logger was passed in.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Severity for a [`Log`] line (parsed from Rhai `log("info", "...")` etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Parse a level string (case-insensitive). Returns [`None`] if unknown.
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "trace" => Some(Self::Trace),
            "debug" => Some(Self::Debug),
            "info" => Some(Self::Info),
            "warn" | "warning" => Some(Self::Warn),
            "error" => Some(Self::Error),
            _ => None,
        }
    }

    /// Same as [`Self::parse`], but falls back to [`LogLevel::Info`].
    pub fn parse_or_info(s: &str) -> Self {
        Self::parse(s).unwrap_or(Self::Info)
    }
}

/// One line recorded from a script (or host code using [`Logger::log`]).
#[derive(Debug, Clone)]
pub struct Log {
    /// Time since the parent [`Logger`] was created.
    pub elapsed: Duration,
    pub level: LogLevel,
    pub message: String,
    /// Source script path or label (e.g. post-script file path).
    pub script: String,
    /// Caller context (e.g. `"post_script"`).
    pub initiator: String,
}

/// Thread-safe collector; cheap to [`Clone`] (shares the same backing storage via [`Arc`]).
#[derive(Debug, Clone)]
pub struct Logger {
    start: Instant,
    logs: Arc<Mutex<Vec<Log>>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Record a message with a known level.
    pub fn log(
        &self,
        level: LogLevel,
        message: impl Into<String>,
        script: impl Into<String>,
        initiator: impl Into<String>,
    ) {
        let entry = Log {
            elapsed: self.start.elapsed(),
            level,
            message: message.into(),
            script: script.into(),
            initiator: initiator.into(),
        };
        if let Ok(mut guard) = self.logs.lock() {
            guard.push(entry);
        }
    }

    /// Parse `level` (case-insensitive); unknown values become [`LogLevel::Info`].
    pub fn log_parsed_level(
        &self,
        level: &str,
        message: impl Into<String>,
        script: impl Into<String>,
        initiator: impl Into<String>,
    ) {
        self.log(LogLevel::parse_or_info(level), message, script, initiator);
    }

    /// Clone of all entries, oldest first.
    pub fn snapshot(&self) -> Vec<Log> {
        self.logs.lock().map(|g| g.clone()).unwrap_or_default()
    }

    /// Remove and return all entries, oldest first.
    pub fn drain(&self) -> Vec<Log> {
        self.logs
            .lock()
            .map(|mut g| std::mem::take(&mut *g))
            .unwrap_or_default()
    }

    pub fn clear(&self) {
        if let Ok(mut g) = self.logs.lock() {
            g.clear();
        }
    }

    pub fn len(&self) -> usize {
        self.logs.lock().map(|g| g.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

use std::path::Path;

use colored::Colorize;
use nd_constants::TRACING_TARGET_RHAI;

/// Severity for a [`Log`] line (parsed from Rhai `log("info", "...")` etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, strum::Display)]
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

    pub fn as_str(&self) -> &str {
        return match self {
            LogLevel::Debug => "debug",
            LogLevel::Error => "error",
            LogLevel::Info => "info",
            LogLevel::Trace => "trace",
            LogLevel::Warn => "warn",
        };
    }
}

/// Parse `level` (case-insensitive); unknown values become [`LogLevel::Info`].
pub fn log_parsed_level(level: &str, message: impl Into<String>, script: impl Into<String>) {
    let msg = message.into();
    let src = script_file_name(&script.into());
    let parsed = LogLevel::parse_or_info(level);

    let level_colored = match parsed {
        LogLevel::Trace => level.cyan(),
        LogLevel::Debug => level.blue(),
        LogLevel::Info => level.green(),
        LogLevel::Warn => level.yellow(),
        LogLevel::Error => level.red(),
    };

    let src_colored = src.color("#444444");

    println!("[{level_colored}・{src_colored}] {msg}");
}

fn script_file_name(script: &str) -> String {
    Path::new(script)
        .file_name()
        .and_then(|s| s.to_str())
        .map(String::from)
        .unwrap_or_else(|| script.to_string())
}

/// Emits a structured [`tracing`] event for one script log line (independent of in-memory capture).
pub fn emit_script_log_to_tracing(level: LogLevel, script: &str, message: &str) {
    let script = script_file_name(script);

    match level {
        LogLevel::Trace => {
            tracing::trace!(target: TRACING_TARGET_RHAI, %script, %message);
        }
        LogLevel::Debug => {
            tracing::debug!(target: TRACING_TARGET_RHAI, %script, %message);
        }
        LogLevel::Info => {
            tracing::info!(target: TRACING_TARGET_RHAI, %script, %message);
        }
        LogLevel::Warn => {
            tracing::warn!(target: TRACING_TARGET_RHAI, %script, %message);
        }
        LogLevel::Error => {
            tracing::error!(target: TRACING_TARGET_RHAI, %script, %message);
        }
    }
}

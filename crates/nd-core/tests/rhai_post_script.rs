//! Tests for [`nd_core::run_post_script`] and optional [`nd_core::Logger`].

use std::sync::Arc;

use nd_core::{run_post_script, LogLevel, Logger, RuntimeEnv};
use tempfile::tempdir;

#[test]
fn post_script_sets_runtime_from_json_body() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("t.rhai");
    std::fs::write(&script, r#"let j = json(); if j != () { set("K", j.id); }"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    run_post_script(
        &script,
        &env,
        200,
        &[("Content-Type".into(), "application/json".into())],
        br#"{"id":"xyz"}"#,
        None,
    )
    .unwrap();
    assert_eq!(env.get("K").as_deref(), Some("xyz"));
}

#[test]
fn post_script_log_fn_requires_logger() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("log.rhai");
    std::fs::write(&script, r#"log("warn", "x");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    let err = run_post_script(&script, &env, 200, &[], b"", None).unwrap_err();
    assert!(
        err.to_string().contains("log") || err.to_string().contains("function"),
        "{err}"
    );
}

#[test]
fn post_script_logger_collects_log_calls() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("log.rhai");
    std::fs::write(&script, r#"log("debug", "step 1"); log("error", "done");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    let logger = Arc::new(Logger::new());
    run_post_script(&script, &env, 200, &[], b"", Some(logger.clone())).unwrap();
    let logs = logger.snapshot();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].level, LogLevel::Debug);
    assert_eq!(logs[0].message, "step 1");
    assert_eq!(logs[1].level, LogLevel::Error);
    assert_eq!(logs[1].message, "done");
    assert!(logs[0].script.ends_with("log.rhai"));
    assert_eq!(logs[0].initiator, "post_script");
}

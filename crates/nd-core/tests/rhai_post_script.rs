//! Tests for [`nd_core::run_post_script`] and optional [`nd_core::Logger`].

use std::sync::Arc;

use nd_constants::RUNTIME_PERSIST_FILENAME;
use nd_core::rhai::logger::{LogLevel, Logger};
use nd_core::{run_post_script, Error, RuntimeEnv};
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
        None,
    )
    .unwrap();
    assert_eq!(env.get("K").as_deref(), Some("xyz"));
}

#[test]
fn post_script_log_fn_without_logger_ok() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("log.rhai");
    std::fs::write(&script, r#"log("warn", "x");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    run_post_script(&script, &env, 200, &[], b"", None, None).unwrap();
}

#[test]
fn post_script_logger_collects_log_calls() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("log.rhai");
    std::fs::write(&script, r#"log("debug", "step 1"); log("error", "done");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    let logger = Arc::new(Logger::new());
    run_post_script(&script, &env, 200, &[], b"", Some(logger.clone()), None).unwrap();
    let logs = logger.snapshot();
    assert_eq!(logs.len(), 2);
    assert_eq!(logs[0].level, LogLevel::Debug);
    assert_eq!(logs[0].message, "step 1");
    assert_eq!(logs[1].level, LogLevel::Error);
    assert_eq!(logs[1].message, "done");
    assert!(logs[0].script.ends_with("log.rhai"));
    assert_eq!(logs[0].initiator, "post_script");
}

#[test]
fn post_script_assert_true_succeeds() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("assert_ok.rhai");
    std::fs::write(&script, r#"assert(true, "should not run");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    run_post_script(&script, &env, 200, &[], b"", None, None).unwrap();
}

#[test]
fn post_script_assert_false_fails_with_message() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("assert_fail.rhai");
    std::fs::write(&script, r#"assert(false, "my message");"#).unwrap();
    let env = RuntimeEnv::from_process_env();
    let err = run_post_script(&script, &env, 200, &[], b"", None, None).unwrap_err();
    match err {
        Error::Rhai(s) => {
            assert!(s.contains("assertion failed"), "{s}");
            assert!(s.contains("my message"), "{s}");
        }
        other => panic!("expected Error::Rhai, got {other:?}"),
    }
}

#[test]
fn post_script_persist_round_trip() {
    let dir = tempdir().unwrap();
    let persist_path = dir.path().join(RUNTIME_PERSIST_FILENAME);
    std::fs::write(&persist_path, r#"{"EXISTING":"from_file"}"#).unwrap();

    let env = RuntimeEnv::isolated();
    env.merge_runtime_persist_file(&persist_path).unwrap();
    assert_eq!(env.get("EXISTING").as_deref(), Some("from_file"));

    let script = dir.path().join("p.rhai");
    std::fs::write(&script, r#"persist("NEW", "stored");"#).unwrap();

    run_post_script(
        &script,
        &env,
        200,
        &[],
        b"",
        None,
        Some(persist_path.clone()),
    )
    .unwrap();

    assert_eq!(env.get("NEW").as_deref(), Some("stored"));
    let text = std::fs::read_to_string(&persist_path).unwrap();
    assert!(text.contains("EXISTING"));
    assert!(text.contains("NEW"));
    assert!(text.contains("stored"));
}

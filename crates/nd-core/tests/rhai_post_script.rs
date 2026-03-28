//! Tests for [`nd_core::rhai_host::run_post_script`].

use nd_core::rhai_host::run_post_script;
use nd_core::RuntimeEnv;
use tempfile::tempdir;

#[test]
fn post_script_sets_runtime_from_json_body() {
    let dir = tempdir().unwrap();
    let script = dir.path().join("t.rhai");
    std::fs::write(
        &script,
        r#"let j = response_body_json(); set_runtime("K", j.id);"#,
    )
    .unwrap();
    let env = RuntimeEnv::from_process_env();
    run_post_script(
        &script,
        &env,
        200,
        &[("Content-Type".into(), "application/json".into())],
        br#"{"id":"xyz"}"#,
    )
    .unwrap();
    assert_eq!(env.get("K").as_deref(), Some("xyz"));
}

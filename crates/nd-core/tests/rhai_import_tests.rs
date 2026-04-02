//! Integration tests for Rhai `import` (`.rhai` + request files) and `call()`.

use std::sync::Arc;

use nd_core::env::RuntimeEnv;
use nd_core::rhai::{run_rhai_script, Logger, RhaiScriptRunOptions};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn rhai_imports_sibling_rhai_module() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("utils.rhai"),
        "fn double(x) { x * 2 }\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("main.rhai"),
        r#"import "utils.rhai" as u;
assert(u::double(21) == 42, "double");
"#,
    )
    .unwrap();

    let main = dir.path().join("main.rhai");
    run_rhai_script(
        &main,
        &RuntimeEnv::new(),
        None,
        RhaiScriptRunOptions::default(),
    )
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rhai_import_request_call_overrides_runtime_env() {
    let dir = tempfile::tempdir().unwrap();
    let server = MockServer::start().await;

    let req_json = format!(
        r#"{{
  "version": "0.1.1",
  "name": "t",
  "request": {{
    "method": "GET",
    "url": "{0}/users/${{ID}}",
    "headers": {{}},
    "follow_redirects": true,
    "verify_tls": true
  }}
}}"#,
        server.uri()
    );
    std::fs::write(dir.path().join("req.json"), req_json).unwrap();

    std::fs::write(
        dir.path().join("main.rhai"),
        r#"import "req.json" as api;
let r = api::invoke(#{ ID: "42" });
assert(r.status == 200, "status");
"#,
    )
    .unwrap();

    Mock::given(method("GET"))
        .and(path("/users/42"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;

    let env = RuntimeEnv::new();
    env.set("ID", "999");

    let main = dir.path().join("main.rhai");
    run_rhai_script(
        &main,
        &env,
        Some(Arc::new(Logger::new())),
        RhaiScriptRunOptions {
            no_network_io: false,
        },
    )
    .unwrap();
}

#[test]
fn rhai_call_no_network_io_does_not_hit_wire() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("req.json"),
        r#"{
  "version": "0.1.1",
  "name": "dry",
  "request": {
    "method": "GET",
    "url": "https://example.invalid/no-network",
    "headers": {},
    "follow_redirects": true,
    "verify_tls": true
  }
}"#,
    )
    .unwrap();
    std::fs::write(
        dir.path().join("main.rhai"),
        r#"import "req.json" as api;
let r = api::invoke(#{});
assert(r.dry_run == true, "dry_run");
assert(r.final_url == "https://example.invalid/no-network", "url");
"#,
    )
    .unwrap();

    let main = dir.path().join("main.rhai");
    run_rhai_script(
        &main,
        &RuntimeEnv::new(),
        None,
        RhaiScriptRunOptions {
            no_network_io: true,
        },
    )
    .unwrap();
}

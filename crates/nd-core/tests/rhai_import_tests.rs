//! Integration tests for Rhai `import` (`.rhai` + request files) and `call()`.

use std::sync::{Arc, Mutex};

use nd_core::env::RuntimeEnv;
use nd_core::rhai::{resolver::RhaiScriptRunOptions, run::run_rhai_script};
use nd_core::stream::Session;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn rhai_imports_sibling_rhai_module() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("utils.rhai"), "fn double(x) { x * 2 }\n").unwrap();
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
        Arc::new(Mutex::new(
            Session::new(|| Ok(RuntimeEnv::new()), None).unwrap(),
        )),
        RhaiScriptRunOptions::default(),
    )
    .unwrap();
}

#[test]
fn rhai_relative_imports() {
    // tests that a script's imports is relative to the script importing it.
    // utils/a.rahi
    // script/b.rhai
    // ext/c.rhai
    // req/a.yaml
    //
    // b.rhai:
    // import "../utils/a" as a;
    //
    // a.rhai:
    // import "../ext/c" as c;
    // import "../req/a.yaml" as a;

    // ...

    // let tmp = tempfile::tempdir().unwrap();
    // let util_dir = tmp.path().join("utils");
    // let request_dir = tmp.path().join("requests");
    // let scripts_dir = tmp.path().join("scripts");

    // std::fs::create_dir(request_dir.clone()).unwrap();
    // std::fs::create_dir(util_dir.clone()).unwrap();
    // std::fs::create_dir(scripts_dir.clone()).unwrap();

    // std::fs::write(
    //     util_dir.join("helper.rhai"),
    //     r#"
    //     "#,
    // )
    // .unwrap();
    // std::fs::write(scripts_dir.join("main.rhai"), r#""#).unwrap();

    // run_rhai_script(
    //     &scripts_dir.join("main.rhai"),
    //     Arc::new(Mutex::new(
    //         Session::new(|| Ok(RuntimeEnv::new()), None).unwrap(),
    //     )),
    //     RhaiScriptRunOptions::default(),
    // )
    // .unwrap();
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
        Arc::new(Mutex::new(Session::new(|| Ok(env.clone()), None).unwrap())),
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
        Arc::new(Mutex::new(
            Session::new(|| Ok(RuntimeEnv::new()), None).unwrap(),
        )),
        RhaiScriptRunOptions {
            no_network_io: true,
        },
    )
    .unwrap();
}

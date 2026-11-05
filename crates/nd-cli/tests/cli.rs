//! Runs the real binary against a mock server.

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use serde_json::json;
use tempfile::TempDir;
use wiremock::matchers::{body_json, body_string_contains, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SECRET: &str = "s3cr3t-value-123";

fn nativedoctor(cwd: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_nativedoctor"));
    command.current_dir(cwd);
    command.env_remove("API_TOKEN");
    return command;
}

fn run(cwd: &Path, args: &[&str]) -> Output {
    return nativedoctor(cwd)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .unwrap();
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn code(output: &Output) -> i32 {
    output.status.code().unwrap()
}

/// A collection whose `dev` environment points `BASE_URL` at `server`.
fn collection(server: &MockServer, secrets: &[&str]) -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    let init = run(dir.path(), &["init", "."]);
    assert_eq!(code(&init), 0, "{}", stderr(&init));
    fs::remove_file(dir.path().join("ping.request.yaml")).unwrap();
    let env = json!({ "vars": { "BASE_URL": server.uri() }, "secrets": secrets });
    fs::write(dir.path().join("environments/dev.yaml"), env.to_string()).unwrap();
    return dir;
}

fn write(dir: &TempDir, name: &str, text: &str) {
    let path = dir.path().join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

async fn respond(server: &MockServer, route: &str, response: ResponseTemplate) {
    Mock::given(path(route))
        .respond_with(response)
        .mount(server)
        .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn piped_output_is_the_raw_body_only() {
    let server = MockServer::start().await;
    let raw = "{\"z\":1.10,  \"a\":2}";
    respond(
        &server,
        "/x",
        ResponseTemplate::new(200).set_body_string(raw),
    )
    .await;

    let dir = tempfile::tempdir().unwrap();
    let output = run(dir.path(), &["send", &format!("{}/x", server.uri())]);
    assert_eq!(code(&output), 0);
    assert_eq!(stdout(&output), raw);
}

#[tokio::test(flavor = "multi_thread")]
async fn pretty_output_keeps_order_and_number_text() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/x",
        ResponseTemplate::new(200).set_body_string("{\"z\":1.10,\"a\":2}"),
    )
    .await;

    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/x", server.uri());
    let output = run(dir.path(), &["send", &url, "--pretty", "all"]);
    let text = stdout(&output);
    assert!(text.contains("200 OK"), "{text}");
    assert!(text.contains("\"z\": 1.10,\n  \"a\": 2"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn send_builds_requests_from_items() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users"))
        .and(query_param("page", "2"))
        .and(header("x-debug", "1"))
        .and(body_json(
            json!({ "name": "John", "age": 29, "tags": ["a"], "user": { "city": "Lagos" } }),
        ))
        .respond_with(ResponseTemplate::new(201))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/users", server.uri());
    let output = run(
        dir.path(),
        &[
            "send",
            &url,
            "name=John",
            "age:=29",
            "tags[]=a",
            "user[city]=Lagos",
            "X-Debug:1",
            "page==2",
        ],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));
}

#[tokio::test(flavor = "multi_thread")]
async fn items_change_a_request_file_for_one_run() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/users"))
        .and(header("x-env", "dev"))
        .and(body_json(json!({ "name": "Jane", "keep": true })))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = collection(&server, &[]);
    let text = "http:\n  method: PUT\n  url: ${BASE_URL}/users\n  headers: { x-env: prod }\n  body:\n    json: { name: John, keep: true }\n";
    write(&dir, "u.request.yaml", text);
    let output = run(
        dir.path(),
        &["run", "u.request.yaml", "name=Jane", "x-env:dev"],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));

    let saved = fs::read_to_string(dir.path().join("u.request.yaml")).unwrap();
    assert_eq!(saved, text, "items must not change the file");
}

#[tokio::test(flavor = "multi_thread")]
async fn saved_requests_replay_and_never_overwrite() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/users"))
        .and(body_json(json!({ "name": "John" })))
        .and(header("x-who", "local"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = collection(&server, &[]);
    let save = |dir: &TempDir| {
        run(
            dir.path(),
            &[
                "send",
                "${BASE_URL}/users",
                "name=John",
                "X-Who:${WHO}",
                "--var",
                "WHO=local",
                "--offline",
                "--save",
                "users/create.request.yaml",
            ],
        )
    };
    let output = save(&dir);
    assert_eq!(code(&output), 0, "{}", stderr(&output));

    let saved = fs::read_to_string(dir.path().join("users/create.request.yaml")).unwrap();
    assert!(saved.contains("${BASE_URL}/users"), "{saved}");
    assert!(
        saved.contains("${WHO}"),
        "saved the resolved value: {saved}"
    );
    assert!(
        !saved.contains("local"),
        "saved the resolved value: {saved}"
    );

    let replay = run(
        dir.path(),
        &["run", "users/create.request.yaml", "--var", "WHO=local"],
    );
    assert_eq!(code(&replay), 0, "{}", stderr(&replay));

    assert_ne!(code(&save(&dir)), 0, "must refuse to overwrite");
}

#[tokio::test(flavor = "multi_thread")]
async fn undeclared_process_env_is_unreachable() {
    let server = MockServer::start().await;
    Mock::given(path("/leak"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let dir = collection(&server, &[]);
    write(
        &dir,
        "evil.request.yaml",
        "http: { url: '${BASE_URL}/leak?k=${SOME_SECRET}' }\n",
    );
    let output = nativedoctor(dir.path())
        .args(["run", "evil.request.yaml"])
        .env("SOME_SECRET", SECRET)
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert_eq!(code(&output), 1);
    assert!(
        stderr(&output).contains("SOME_SECRET"),
        "{}",
        stderr(&output)
    );
    assert!(!stderr(&output).contains(SECRET));
}

#[tokio::test(flavor = "multi_thread")]
async fn request_files_cant_upload_files_outside_the_collection() {
    let server = MockServer::start().await;
    Mock::given(path("/up"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let dir = collection(&server, &[]);
    let outside = tempfile::tempdir().unwrap();
    let key = outside.path().join("id_rsa");
    fs::write(&key, SECRET).unwrap();
    let text = format!(
        "http:\n  url: ${{BASE_URL}}/up\n  body:\n    file: {}\n",
        key.display()
    );
    write(&dir, "up.request.yaml", &text);

    let output = run(dir.path(), &["run", "up.request.yaml"]);
    assert_eq!(code(&output), 1);
    assert!(
        stderr(&output).contains("outside the collection"),
        "{}",
        stderr(&output)
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn missing_secret_stops_the_request() {
    let server = MockServer::start().await;
    Mock::given(path("/me"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let dir = collection(&server, &["API_TOKEN"]);
    write(
        &dir,
        "me.request.yaml",
        "http:\n  url: ${BASE_URL}/me\n  headers: { authorization: 'Bearer ${API_TOKEN}' }\n",
    );
    let output = run(dir.path(), &["run", "me.request.yaml"]);
    assert_eq!(code(&output), 1);
    assert!(
        stderr(&output).contains(".nativedoctor/secrets/dev.env"),
        "{}",
        stderr(&output)
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn secrets_are_sent_but_never_shown() {
    let server = MockServer::start().await;
    Mock::given(path("/me"))
        .and(header("authorization", format!("Bearer {SECRET}").as_str()))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-echo", SECRET)
                .set_body_string(SECRET),
        )
        .expect(2)
        .mount(&server)
        .await;

    let dir = collection(&server, &["API_TOKEN"]);
    write(
        &dir,
        "me.request.yaml",
        "http:\n  url: ${BASE_URL}/me\n  headers: { authorization: 'Bearer ${API_TOKEN}' }\n",
    );
    write(
        &dir,
        ".nativedoctor/secrets/dev.env",
        &format!("API_TOKEN={SECRET}\n"),
    );

    let output = run(
        dir.path(),
        &["run", "me.request.yaml", "-v", "--pretty", "all"],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));
    let text = stdout(&output);
    assert!(!text.contains(SECRET), "{text}");
    assert!(text.contains("Bearer ***"), "{text}");

    let json = stdout(&run(dir.path(), &["run", "me.request.yaml", "--json"]));
    assert!(!json.contains(SECRET), "{json}");
}

#[tokio::test(flavor = "multi_thread")]
async fn offline_redacts_and_never_connects() {
    let server = MockServer::start().await;
    Mock::given(path("/me"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let dir = collection(&server, &["API_TOKEN"]);
    write(
        &dir,
        "me.request.yaml",
        "http:\n  url: ${BASE_URL}/me\n  headers: { authorization: 'Bearer ${API_TOKEN}' }\n",
    );
    let output = run(
        dir.path(),
        &[
            "run",
            "me.request.yaml",
            "--offline",
            "--var",
            &format!("API_TOKEN={SECRET}"),
        ],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));
    assert!(
        stdout(&output).contains("Bearer ***"),
        "{}",
        stdout(&output)
    );
    assert!(!stdout(&output).contains(SECRET));
}

#[tokio::test(flavor = "multi_thread")]
async fn capture_flows_to_later_requests_and_persists() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/login",
        ResponseTemplate::new(200).set_body_json(json!({ "access_token": SECRET })),
    )
    .await;
    Mock::given(path("/me"))
        .and(header("authorization", format!("Bearer {SECRET}").as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_string("hello"))
        .expect(2)
        .mount(&server)
        .await;

    let dir = collection(&server, &["API_TOKEN"]);
    write(
        &dir,
        "flow/1-login.request.yaml",
        "http: { url: '${BASE_URL}/login' }\ncapture: { API_TOKEN: $.access_token }\nassert: { status: 200 }\n",
    );
    write(
        &dir,
        "flow/2-me.request.yaml",
        "http:\n  url: ${BASE_URL}/me\n  headers: { authorization: 'Bearer ${API_TOKEN}' }\n",
    );

    // Folder runs share captures, in path order.
    let output = run(dir.path(), &["run", "flow", "--pretty", "all"]);
    assert_eq!(code(&output), 0, "{}", stderr(&output));
    assert!(!stdout(&output).contains(SECRET), "{}", stdout(&output));
    assert!(stdout(&output).contains("hello"));

    // A later run picks the token up from state.
    let output = run(dir.path(), &["run", "flow/2-me.request.yaml"]);
    assert_eq!(code(&output), 0, "{}", stderr(&output));

    let state = dir.path().join(".nativedoctor/state/dev.yaml");
    assert!(state.exists());
    let ignore = fs::read_to_string(dir.path().join(".nativedoctor/.gitignore")).unwrap();
    assert_eq!(ignore.trim(), "*");
}

#[tokio::test(flavor = "multi_thread")]
async fn exit_codes_follow_the_agreed_rules() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/ok",
        ResponseTemplate::new(200).set_body_json(json!({ "n": 1 })),
    )
    .await;
    respond(
        &server,
        "/moved",
        ResponseTemplate::new(302).insert_header("location", "/ok"),
    )
    .await;
    respond(&server, "/missing", ResponseTemplate::new(404)).await;
    respond(&server, "/broken", ResponseTemplate::new(500)).await;
    respond(
        &server,
        "/loop",
        ResponseTemplate::new(302).insert_header("location", "/loop"),
    )
    .await;
    respond(
        &server,
        "/slow",
        ResponseTemplate::new(200).set_delay(Duration::from_secs(3)),
    )
    .await;

    let dir = tempfile::tempdir().unwrap();
    let url = |route: &str| format!("{}{route}", server.uri());
    let exit = |args: &[&str]| code(&run(dir.path(), args));

    assert_eq!(
        exit(&["send", &url("/missing")]),
        0,
        "a 404 alone isn't a failure"
    );
    assert_eq!(exit(&["send", &url("/ok"), "--fail"]), 0);
    assert_eq!(exit(&["send", &url("/moved"), "--fail"]), 3);
    assert_eq!(exit(&["send", &url("/missing"), "--fail"]), 4);
    assert_eq!(exit(&["send", &url("/broken"), "--fail"]), 5);
    assert_eq!(exit(&["send", &url("/moved"), "--follow", "--fail"]), 0);
    assert_eq!(
        exit(&["send", &url("/loop"), "--follow", "--max-redirects", "2"]),
        6
    );
    assert_eq!(exit(&["send", &url("/slow"), "--timeout", "200ms"]), 2);
    assert_eq!(exit(&["send", "http://127.0.0.1:1/"]), 1);
    assert_eq!(
        exit(&["send", "--bogus-flag", "x"]),
        1,
        "usage errors must not look like timeouts"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn redirects_arent_followed_by_default() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/moved",
        ResponseTemplate::new(302).insert_header("location", "/ok"),
    )
    .await;
    Mock::given(path("/ok"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/moved", server.uri());
    run(dir.path(), &["send", &url]);
    run(dir.path(), &["send", &url, "--follow"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn failed_assertions_exit_seven_and_explain() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/x",
        ResponseTemplate::new(200).set_body_json(json!({ "name": "John" })),
    )
    .await;

    let dir = collection(&server, &[]);
    write(
        &dir,
        "x.request.yaml",
        "http: { url: '${BASE_URL}/x' }\nassert:\n  status: 201\n  body: { $.name: Jane }\n",
    );
    let output = run(dir.path(), &["run", "x.request.yaml"]);
    assert_eq!(code(&output), 7);
    assert!(
        stderr(&output).contains("expected 201, got 200"),
        "{}",
        stderr(&output)
    );
    assert!(stderr(&output).contains("Jane"), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("John"),
        "the body should still print"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn a_folder_run_stops_at_the_first_failure() {
    let server = MockServer::start().await;
    respond(&server, "/a", ResponseTemplate::new(500)).await;
    Mock::given(path("/b"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let dir = collection(&server, &[]);
    write(
        &dir,
        "1.request.yaml",
        "http: { url: '${BASE_URL}/a' }\nassert: { status: 200 }\n",
    );
    write(&dir, "2.request.yaml", "http: { url: '${BASE_URL}/b' }\n");
    assert_eq!(code(&run(dir.path(), &["run", "."])), 7);
}

#[tokio::test(flavor = "multi_thread")]
async fn stdin_is_only_read_when_asked() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/x",
        ResponseTemplate::new(200).set_body_string("done"),
    )
    .await;

    // An open, silent stdin (like cron or CI) must not hang the request.
    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/x", server.uri());
    let mut child = nativedoctor(dir.path())
        .args(["send", &url])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let held_open = child.stdin.take();
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while child.try_wait().unwrap().is_none() {
        assert!(
            std::time::Instant::now() < deadline,
            "hung waiting for stdin"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    drop(held_open);
}

#[tokio::test(flavor = "multi_thread")]
async fn body_flag_reads_stdin_without_expanding_it() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/x"))
        .and(header("content-type", "application/json"))
        .and(body_string_contains("${NOT_A_VAR}"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/x", server.uri());
    let mut child = nativedoctor(dir.path())
        .args(["send", &url, "--body", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(br#"{"a":"${NOT_A_VAR}"}"#)
        .unwrap();
    assert!(child.wait().unwrap().success());
}

#[tokio::test(flavor = "multi_thread")]
async fn uploads_files_as_multipart() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/up"))
        .and(body_string_contains("file-contents"))
        .and(body_string_contains("name=\"title\""))
        .and(body_string_contains("filename=\"cv.txt\""))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("cv.txt"), "file-contents").unwrap();
    let url = format!("{}/up", server.uri());
    let output = run(
        dir.path(),
        &["send", &url, "--form", "title=hello", "cv@cv.txt"],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));
}

#[tokio::test(flavor = "multi_thread")]
async fn json_output_is_structured() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/x",
        ResponseTemplate::new(200).set_body_json(json!({ "n": 1 })),
    )
    .await;

    let dir = tempfile::tempdir().unwrap();
    let url = format!("{}/x", server.uri());
    let output = run(dir.path(), &["send", &url, "--json"]);
    let value: serde_json::Value = serde_json::from_str(&stdout(&output)).unwrap();
    assert_eq!(value["response"]["status"], 200);
    assert_eq!(value["response"]["body"]["n"], 1);
    assert_eq!(value["request"]["method"], "GET");

    let output = run(dir.path(), &["send", &url, "--json", "--stream"]);
    let events: Vec<serde_json::Value> = stdout(&output)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(events.first().unwrap()["event"], "head");
    assert_eq!(events.last().unwrap()["event"], "finished");
}

#[tokio::test(flavor = "multi_thread")]
async fn list_and_check_report_on_the_collection() {
    let server = MockServer::start().await;
    let dir = collection(&server, &[]);
    write(
        &dir,
        "users/get.request.yaml",
        "name: Get user\nhttp: { url: '${BASE_URL}/u' }\n",
    );
    write(
        &dir,
        "users/new.request.yaml",
        "http: { url: '${BASE_URL}/u', body: { json: {} } }\n",
    );

    let listed = stdout(&run(dir.path(), &["list"]));
    assert!(
        listed.contains("GET") && listed.contains("Get user"),
        "{listed}"
    );
    assert!(
        listed.contains("POST") && listed.contains("users/new.request.yaml"),
        "{listed}"
    );

    assert_eq!(code(&run(dir.path(), &["check"])), 0);

    write(&dir, "bad.request.yaml", "http: { url: '${NOPE}' }\n");
    let output = run(dir.path(), &["check"]);
    assert_eq!(code(&output), 1);
    assert!(stderr(&output).contains("NOPE"), "{}", stderr(&output));
    assert_eq!(code(&run(dir.path(), &["check", "--var", "NOPE=x"])), 0);
}

#[test]
fn planned_protocols_explain_themselves() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("f.request.yaml"), "ws:\n  url: wss://x\n").unwrap();
    let output = run(dir.path(), &["run", "f.request.yaml"]);
    assert_eq!(code(&output), 1);
    assert!(
        stderr(&output).contains("ws requests aren't supported yet"),
        "{}",
        stderr(&output)
    );
}

/// The `Cookie` header on each request the server received for `route`, in order.
async fn cookies_sent(server: &MockServer, route: &str) -> Vec<Option<String>> {
    let requests = server.received_requests().await.unwrap();
    return requests
        .iter()
        .filter(|request| request.url.path() == route)
        .map(|request| {
            request
                .headers
                .get("cookie")
                .map(|value| value.to_str().unwrap().to_string())
        })
        .collect();
}

fn set_cookie(value: &str) -> ResponseTemplate {
    ResponseTemplate::new(200).insert_header("set-cookie", value)
}

#[tokio::test(flavor = "multi_thread")]
async fn cookies_persist_between_runs_in_a_collection() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    write(
        &dir,
        "login.request.yaml",
        "http: { url: '${BASE_URL}/login' }\n",
    );
    write(&dir, "me.request.yaml", "http: { url: '${BASE_URL}/me' }\n");

    assert_eq!(code(&run(dir.path(), &["run", "me.request.yaml"])), 0);
    assert_eq!(code(&run(dir.path(), &["run", "login.request.yaml"])), 0);
    assert_eq!(code(&run(dir.path(), &["run", "me.request.yaml"])), 0);

    assert_eq!(
        cookies_sent(&server, "/me").await,
        vec![None, Some("session=abc123".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn each_environment_has_its_own_cookies() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=dev")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    let env = json!({ "vars": { "BASE_URL": server.uri() } });
    write(&dir, "environments/prod.yaml", &env.to_string());
    write(
        &dir,
        "login.request.yaml",
        "http: { url: '${BASE_URL}/login' }\n",
    );
    write(&dir, "me.request.yaml", "http: { url: '${BASE_URL}/me' }\n");

    run(dir.path(), &["run", "login.request.yaml", "-e", "dev"]);
    run(dir.path(), &["run", "me.request.yaml", "-e", "prod"]);
    run(dir.path(), &["run", "me.request.yaml", "-e", "dev"]);

    assert_eq!(
        cookies_sent(&server, "/me").await,
        vec![None, Some("session=dev".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn saved_cookies_only_go_to_the_host_that_set_them() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    let local = server.uri().replace("127.0.0.1", "localhost");
    let login = format!("{}/login", server.uri());
    run(dir.path(), &["send", &login]);
    let output = run(dir.path(), &["send", &format!("{local}/me")]);
    assert_eq!(code(&output), 0, "{}", stderr(&output));

    assert_eq!(cookies_sent(&server, "/me").await, vec![None]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_folder_run_shares_cookies_without_a_collection() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = tempfile::tempdir().unwrap();
    let uri = server.uri();
    write(
        &dir,
        "1.request.yaml",
        &format!("http: {{ url: '{uri}/login' }}\n"),
    );
    write(
        &dir,
        "2.request.yaml",
        &format!("http: {{ url: '{uri}/me' }}\n"),
    );
    let output = run(dir.path(), &["run", "."]);
    assert_eq!(code(&output), 0, "{}", stderr(&output));

    assert_eq!(
        cookies_sent(&server, "/me").await,
        vec![Some("session=abc123".to_string())]
    );
    assert!(
        !dir.path().join(".nativedoctor").exists(),
        "nowhere to save them"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn no_cookies_neither_sends_nor_saves() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    let login = format!("{}/login", server.uri());
    let me = format!("{}/me", server.uri());

    run(dir.path(), &["send", &login, "--no-cookies"]);
    assert!(!dir
        .path()
        .join(".nativedoctor/state/dev.cookies.json")
        .exists());

    run(dir.path(), &["send", &login]);
    run(dir.path(), &["send", &me, "--no-cookies"]);
    run(dir.path(), &["send", &me]);

    assert_eq!(
        cookies_sent(&server, "/me").await,
        vec![None, Some("session=abc123".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn an_explicit_cookie_header_replaces_saved_cookies() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=saved")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    run(dir.path(), &["send", &format!("{}/login", server.uri())]);
    let output = run(
        dir.path(),
        &[
            "send",
            &format!("{}/me", server.uri()),
            "Cookie:mine=1",
            "-v",
            "--pretty",
            "all",
        ],
    );

    assert_eq!(
        cookies_sent(&server, "/me").await,
        vec![Some("mine=1".to_string())]
    );
    assert!(stdout(&output).contains("mine=1"), "{}", stdout(&output));
    assert!(
        !stdout(&output).contains("session=***"),
        "{}",
        stdout(&output)
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn saved_cookie_values_are_hidden_in_output() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    run(dir.path(), &["send", &format!("{}/login", server.uri())]);
    let me = format!("{}/me", server.uri());

    for args in [
        vec!["send", &me, "-v", "--pretty", "all"],
        vec!["send", &me, "--offline"],
    ] {
        let text = stdout(&run(dir.path(), &args));
        assert!(text.contains("session=***"), "{text}");
        assert!(!text.contains("abc123"), "{text}");
    }
    let text = stdout(&run(dir.path(), &["send", &me, "--json"]));
    assert!(text.contains("session=***"), "{text}");
    assert!(!text.contains("abc123"), "{text}");

    let text = stdout(&run(dir.path(), &["cookies", "list"]));
    assert!(text.contains("session"), "{text}");
    assert!(!text.contains("abc123"), "{text}");

    // Offline doesn't touch the jar, and the value did go out on the wire.
    let sent = cookies_sent(&server, "/me").await;
    assert_eq!(sent, vec![Some("session=abc123".to_string()); 2]);
}

#[tokio::test(flavor = "multi_thread")]
async fn cookies_clear_logs_out_one_environment() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;
    respond(&server, "/me", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    run(dir.path(), &["send", &format!("{}/login", server.uri())]);
    let output = run(dir.path(), &["cookies", "clear"]);
    assert_eq!(code(&output), 0);
    assert!(stdout(&output).contains("Cleared 1"), "{}", stdout(&output));

    run(dir.path(), &["send", &format!("{}/me", server.uri())]);
    assert_eq!(cookies_sent(&server, "/me").await, vec![None]);

    let again = run(dir.path(), &["cookies", "clear"]);
    assert!(stdout(&again).contains("Cleared 0"), "{}", stdout(&again));
}

#[tokio::test(flavor = "multi_thread")]
async fn cookies_set_on_a_redirect_reach_the_next_request() {
    let server = MockServer::start().await;
    respond(
        &server,
        "/login",
        ResponseTemplate::new(302)
            .insert_header("location", "/home")
            .insert_header("set-cookie", "session=abc123"),
    )
    .await;
    respond(&server, "/home", ResponseTemplate::new(200)).await;

    let dir = collection(&server, &[]);
    let output = run(
        dir.path(),
        &["send", &format!("{}/login", server.uri()), "--follow"],
    );
    assert_eq!(code(&output), 0, "{}", stderr(&output));
    assert_eq!(
        cookies_sent(&server, "/home").await,
        vec![Some("session=abc123".to_string())]
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn cookies_are_saved_when_an_assertion_fails() {
    let server = MockServer::start().await;
    respond(&server, "/login", set_cookie("session=abc123")).await;

    let dir = collection(&server, &[]);
    write(
        &dir,
        "l.request.yaml",
        "http: { url: '${BASE_URL}/login' }\nassert: { status: 201 }\n",
    );
    assert_eq!(code(&run(dir.path(), &["run", "l.request.yaml"])), 7);
    assert!(dir
        .path()
        .join(".nativedoctor/state/dev.cookies.json")
        .exists());
}

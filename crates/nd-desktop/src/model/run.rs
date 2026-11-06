//! Runs requests on the tokio runtime and reports progress to the UI over a channel.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, Weak};
use std::time::Duration;

use nd_core::items::{self, BodyMode, Item};
use nd_core::{Collection, PreparedBody, PreparedRequest, Redactor, RequestFile};
use nd_engine::report::{body_text, request_json, result_json};
use nd_engine::{CookieJar, Event, FileScope, ResponseHead, Session};
use serde_json::Value;
use tokio::sync::mpsc;

const CHANNEL_SIZE: usize = 32;

/// Running requests share a jar per cookie file, so one finishing can't overwrite another's.
static JARS: LazyLock<Mutex<HashMap<PathBuf, Weak<CookieJar>>>> = LazyLock::new(Default::default);
/// `state::save` reads then writes, so saves take turns.
static STATE: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone)]
pub enum Source {
    /// The editor's current contents, saved or not.
    Draft(Box<RequestFile>),
    File(PathBuf),
}

#[derive(Debug, Clone)]
pub struct Target {
    pub name: String,
    pub source: Source,
    /// One-time changes, as on the CLI's `run` and `send`.
    pub items: Vec<Item>,
    pub files: FileScope,
}

/// What stays the same for every request in a run.
#[derive(Debug, Clone, Default)]
pub struct RunInput {
    pub root: Option<PathBuf>,
    pub env: Option<String>,
    pub overrides: Vec<(String, String)>,
    pub use_cookies: bool,
    /// Treat a 3xx, 4xx or 5xx response as a failure, like `--fail`.
    pub fail_on_status: bool,
    /// Show the body as it arrives, even if it isn't an event stream.
    pub stream: bool,
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FailureKind {
    Timeout,
    Redirects,
    Connection,
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Passed,
    Assertions(Vec<String>),
    BadStatus(u16),
    Error { kind: FailureKind, message: String },
}

impl Outcome {
    pub fn is_pass(&self) -> bool {
        matches!(self, Outcome::Passed)
    }
}

/// Text of the request as sent. Secrets are already hidden.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SentRequest {
    pub method: String,
    pub url: String,
    pub head: String,
    pub body: Option<String>,
}

#[derive(Debug)]
pub enum Update {
    Started {
        index: usize,
        name: String,
    },
    Prepared {
        index: usize,
        sent: SentRequest,
        redactor: Redactor,
    },
    Head {
        index: usize,
        head: ResponseHead,
        live: bool,
    },
    Chunk {
        index: usize,
        bytes: Vec<u8>,
    },
    Done {
        index: usize,
        result: Box<RunResult>,
    },
    /// The run is over. Ran fewer than `targets` if one failed.
    Finished,
}

/// The end of one request.
#[derive(Debug, Clone)]
pub struct RunResult {
    pub outcome: Outcome,
    pub body: Arc<Vec<u8>>,
    pub elapsed: Option<Duration>,
    /// Includes captured secrets, unlike the one sent with `Prepared`.
    pub redactor: Redactor,
    pub captured: Vec<String>,
    pub warnings: Vec<String>,
    /// The whole exchange, already redacted.
    pub json: Option<Value>,
}

impl RunResult {
    fn error(kind: FailureKind, message: String, redactor: Redactor) -> RunResult {
        return RunResult {
            outcome: Outcome::Error {
                kind,
                message: redactor.redact(&message),
            },
            body: Arc::default(),
            elapsed: None,
            redactor,
            captured: Vec::new(),
            warnings: Vec::new(),
            json: None,
        };
    }
}

/// Runs each target in order, stopping at the first failure.
pub async fn run(input: RunInput, updates: mpsc::UnboundedSender<Update>) {
    let mut session = match open_session(&input) {
        Ok(session) => session,
        Err(message) => {
            let result = RunResult::error(FailureKind::Other, message, Redactor::default());
            let _ = updates.send(Update::Started {
                index: 0,
                name: input.targets.first().map_or(String::new(), |t| t.name.clone()),
            });
            let _ = updates.send(Update::Done {
                index: 0,
                result: Box::new(result),
            });
            let _ = updates.send(Update::Finished);
            return;
        }
    };

    for (index, target) in input.targets.iter().enumerate() {
        let _ = updates.send(Update::Started {
            index,
            name: target.name.clone(),
        });
        let result = run_target(index, target, &input, &mut session, &updates).await;
        let passed = result.outcome.is_pass();
        let _ = updates.send(Update::Done {
            index,
            result: Box::new(result),
        });
        if !passed {
            break;
        }
    }
    let _ = updates.send(Update::Finished);
}

/// The request a target would send, without sending it. Secrets are hidden.
pub fn preview(input: &RunInput) -> Result<SentRequest, String> {
    let target = input.targets.first().ok_or("nothing to preview")?;
    let session = open_session(input)?;
    let redactor = session.vars.redactor();
    let (_, request) = build(target, &session).map_err(|e| redactor.redact(&e))?;
    return Ok(sent_request(&request, &session, &redactor));
}

fn open_session(input: &RunInput) -> Result<Session, String> {
    let collection = match &input.root {
        Some(root) => Some(Collection::open(root).map_err(|e| e.to_string())?),
        None => None,
    };
    let process_env = |name: &str| std::env::var(name).ok();
    let mut session = Session::open(
        collection,
        input.env.as_deref(),
        &input.overrides,
        input.use_cookies,
        &process_env,
    )
    .map_err(|e| e.to_string())?;
    share_jar(&mut session);
    return Ok(session);
}

fn share_jar(session: &mut Session) {
    let Some(collection) = &session.collection else {
        return;
    };
    let Some(own) = session.jar() else {
        return;
    };
    let path = collection.cookies_path(session.env.as_deref());
    let mut jars = JARS.lock().expect("jar pool");
    jars.retain(|_, jar| jar.strong_count() > 0);
    if let Some(running) = jars.get(&path).and_then(Weak::upgrade) {
        session.share_jar(running);
        return;
    }
    jars.insert(path, Arc::downgrade(&own));
}

fn build(target: &Target, session: &Session) -> Result<(RequestFile, PreparedRequest), String> {
    let mut file = match &target.source {
        Source::Draft(file) => (**file).clone(),
        Source::File(path) => RequestFile::load(path).map_err(|e| e.to_string())?,
    };
    let mut files = target.files.clone();
    let uploads = items::apply(&mut file.http, &target.items, BodyMode::Auto)
        .map_err(|e| e.to_string())?;
    files.allowed.extend(uploads);
    let request = session
        .prepare(&file.http, &files)
        .map_err(|e| e.to_string())?;
    return Ok((file, request));
}

fn sent_request(request: &PreparedRequest, session: &Session, redactor: &Redactor) -> SentRequest {
    let mut target = request.url.path().to_string();
    if let Some(query) = request.url.query() {
        target = format!("{target}?{query}");
    }
    let mut lines = vec![format!("{} {target}", request.method)];
    if let Some(host) = request.url.host_str() {
        let port = request.url.port().map_or(String::new(), |p| format!(":{p}"));
        lines.push(format!("host: {host}{port}"));
    }
    for (name, value) in &request.headers {
        lines.push(format!("{name}: {value}"));
    }
    if let Some(cookies) = session.masked_cookies(request) {
        lines.push(format!("cookie: {cookies}"));
    }
    let body = match &request.body {
        PreparedBody::None => None,
        PreparedBody::Bytes(bytes) => Some(body_text(bytes, true)),
        PreparedBody::Multipart(parts) => Some(
            parts
                .iter()
                .map(|part| match &part.filename {
                    Some(name) => format!("{}: file {name} ({} bytes)", part.name, part.data.len()),
                    None => format!("{}: {}", part.name, String::from_utf8_lossy(&part.data)),
                })
                .collect::<Vec<_>>()
                .join("\n"),
        ),
    };
    return SentRequest {
        method: request.method.clone(),
        url: redactor.redact(request.url.as_str()),
        head: redactor.redact(&lines.join("\n")),
        body: body.map(|text| redactor.redact(&text)),
    };
}

async fn run_target(
    index: usize,
    target: &Target,
    input: &RunInput,
    session: &mut Session,
    updates: &mpsc::UnboundedSender<Update>,
) -> RunResult {
    let redactor = session.vars.redactor();
    let (file, request) = match build(target, session) {
        Ok(built) => built,
        Err(message) => return RunResult::error(FailureKind::Other, message, redactor),
    };
    let sent = sent_request(&request, session, &redactor);
    let cookie_line = session.masked_cookies(&request);
    let _ = updates.send(Update::Prepared {
        index,
        sent,
        redactor: redactor.clone(),
    });

    let (sender, mut events) = mpsc::channel(CHANNEL_SIZE);
    let engine = tokio::spawn(nd_engine::send(request.clone(), session.jar(), sender));

    let mut head: Option<ResponseHead> = None;
    let mut body: Vec<u8> = Vec::new();
    let mut live = input.stream;
    let mut elapsed = None;
    while let Some(event) = events.recv().await {
        match event {
            Event::Head {
                status,
                reason,
                version,
                headers,
            } => {
                let is_event_stream = headers.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case("content-type")
                        && value.starts_with("text/event-stream")
                });
                live = live || is_event_stream;
                let response = ResponseHead {
                    status,
                    reason,
                    version,
                    headers,
                };
                let _ = updates.send(Update::Head {
                    index,
                    head: response.clone(),
                    live,
                });
                head = Some(response);
            }
            Event::Chunk(bytes) => {
                body.extend_from_slice(&bytes);
                if live {
                    let _ = updates.send(Update::Chunk {
                        index,
                        bytes: bytes.to_vec(),
                    });
                }
            }
            Event::Finished { elapsed: time } => elapsed = Some(time),
        }
    }

    let sent_result = engine.await;
    // Saved even after a failed check, since the server has already set them.
    let cookie_warning = session.save_cookies().err().map(|e| e.to_string());
    match sent_result {
        Ok(Ok(())) => {}
        Ok(Err(error)) => {
            let (kind, message) = classify(&error);
            return RunResult::error(kind, message, redactor);
        }
        Err(error) => {
            return RunResult::error(FailureKind::Other, error.to_string(), redactor);
        }
    }
    let Some(head) = head else {
        return RunResult::error(FailureKind::Other, "no response received".to_string(), redactor);
    };

    let mut result = conclude(&file, &head, body, elapsed, session, input);
    result.warnings.extend(cookie_warning);
    if let Some(json) = &mut result.json {
        let sent = request_json(&request, cookie_line.as_deref());
        let failures = match &result.outcome {
            Outcome::Assertions(failures) => failures.as_slice(),
            _ => &[],
        };
        *json = result.redactor.redact_json(&result_json(
            &target.name,
            sent,
            &head,
            &result.body,
            elapsed.map(|time| time.as_millis()),
            failures,
        ));
    }
    return result;
}

fn conclude(
    file: &RequestFile,
    head: &ResponseHead,
    body: Vec<u8>,
    elapsed: Option<Duration>,
    session: &mut Session,
    input: &RunInput,
) -> RunResult {
    let mut result = RunResult {
        outcome: Outcome::Passed,
        body: Arc::new(body),
        elapsed,
        redactor: session.vars.redactor(),
        captured: Vec::new(),
        warnings: Vec::new(),
        json: Some(Value::Null),
    };
    let conclusion = match session.conclude(file, head, &result.body) {
        Ok(conclusion) => conclusion,
        Err(error) => {
            result.outcome = Outcome::Error {
                kind: FailureKind::Other,
                message: result.redactor.redact(&error.to_string()),
            };
            return result;
        }
    };

    // Rebuilt, so captured secrets are hidden from here on.
    result.redactor = session.vars.redactor();
    result.captured = conclusion.saved.iter().map(|(name, _)| name.clone()).collect();
    for name in &conclusion.ignored {
        result
            .warnings
            .push(format!("capture `{name}` ignored; an override or the environment wins"));
    }
    {
        let _turn = STATE.lock().expect("state lock");
        if let Err(error) = session.save_captures(&conclusion.saved) {
            result.warnings.push(result.redactor.redact(&error.to_string()));
        }
    }

    result.outcome = if !conclusion.failures.is_empty() {
        let failures = conclusion.failures.iter();
        Outcome::Assertions(failures.map(|f| result.redactor.redact(f)).collect())
    } else if let Some(error) = conclusion.capture_error {
        Outcome::Error {
            kind: FailureKind::Other,
            message: result.redactor.redact(&error.to_string()),
        }
    } else if input.fail_on_status && (300..600).contains(&head.status) {
        Outcome::BadStatus(head.status)
    } else {
        Outcome::Passed
    };
    return result;
}

fn classify(error: &nd_engine::Error) -> (FailureKind, String) {
    let kind = match error {
        nd_engine::Error::Timeout(_) => FailureKind::Timeout,
        nd_engine::Error::TooManyRedirects(_) => FailureKind::Redirects,
        nd_engine::Error::Connect(_) => FailureKind::Connection,
        _ => FailureKind::Other,
    };
    return (kind, error.to_string());
}

/// A target for a saved file. Its files must stay inside the collection.
pub fn file_target(path: &Path, root: Option<&Path>) -> Target {
    let name = RequestFile::load(path).map_or_else(
        |_| RequestFile::default().display_name(path),
        |file| file.display_name(path),
    );
    return Target {
        name,
        source: Source::File(path.to_path_buf()),
        items: Vec::new(),
        files: saved_scope(path, root),
    };
}

/// A saved request can only read files inside its collection, or its own folder without one.
pub fn saved_scope(path: &Path, root: Option<&Path>) -> FileScope {
    let base_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
    let root = root.map_or_else(|| base_dir.clone(), Path::to_path_buf);
    return FileScope {
        base_dir,
        root: Some(root),
        allowed: Vec::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use nd_core::{Assertions, HttpSpec};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn draft(name: &str, url: String) -> Target {
        let file = RequestFile {
            http: HttpSpec {
                url,
                ..HttpSpec::default()
            },
            ..RequestFile::default()
        };
        return draft_file(name, file);
    }

    fn draft_file(name: &str, file: RequestFile) -> Target {
        return Target {
            name: name.to_string(),
            source: Source::Draft(Box::new(file)),
            items: Vec::new(),
            files: FileScope::default(),
        };
    }

    async fn collect(input: RunInput) -> Vec<Update> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        run(input, sender).await;
        let mut updates = Vec::new();
        while let Some(update) = receiver.recv().await {
            updates.push(update);
        }
        return updates;
    }

    fn results(updates: &[Update]) -> Vec<&RunResult> {
        return updates
            .iter()
            .filter_map(|update| match update {
                Update::Done { result, .. } => Some(&**result),
                _ => None,
            })
            .collect();
    }

    fn input(targets: Vec<Target>) -> RunInput {
        return RunInput {
            use_cookies: true,
            targets,
            ..RunInput::default()
        };
    }

    fn error_message(result: &RunResult) -> String {
        let Outcome::Error { message, .. } = &result.outcome else {
            panic!("expected an error, got {:?}", result.outcome);
        };
        return message.clone();
    }

    #[tokio::test]
    async fn reports_a_complete_exchange() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{\"ok\":1}"))
            .mount(&server)
            .await;
        let updates = collect(input(vec![draft("a", format!("{}/a", server.uri()))])).await;

        assert!(matches!(updates.last(), Some(Update::Finished)));
        let done = results(&updates);
        assert_eq!(done[0].outcome, Outcome::Passed);
        assert_eq!(done[0].body.as_slice(), b"{\"ok\":1}");
        assert!(done[0].elapsed.is_some());
        let json = done[0].json.as_ref().unwrap();
        assert_eq!(json["response"]["status"], 200);
        assert_eq!(json["response"]["body"]["ok"], 1);
    }

    #[tokio::test]
    async fn a_run_stops_at_the_first_failure() {
        let server = MockServer::start().await;
        Mock::given(path("/ok"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        Mock::given(path("/bad"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let mut failing = draft("bad", format!("{}/bad", server.uri()));
        if let Source::Draft(file) = &mut failing.source {
            file.assertions = Assertions {
                status: vec![200],
                ..Assertions::default()
            };
        }
        let targets = vec![
            draft("ok", format!("{}/ok", server.uri())),
            failing,
            draft("never", format!("{}/ok", server.uri())),
        ];
        let updates = collect(input(targets)).await;
        let done = results(&updates);
        assert_eq!(done.len(), 2);
        assert!(done[0].outcome.is_pass());
        assert!(matches!(&done[1].outcome, Outcome::Assertions(f) if f.len() == 1));
    }

    #[tokio::test]
    async fn fail_on_status_only_applies_when_asked() {
        let server = MockServer::start().await;
        Mock::given(path("/x"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let target = draft("x", format!("{}/x", server.uri()));

        let updates = collect(input(vec![target.clone()])).await;
        assert!(results(&updates)[0].outcome.is_pass());

        let mut strict = input(vec![target]);
        strict.fail_on_status = true;
        let updates = collect(strict).await;
        assert_eq!(results(&updates)[0].outcome, Outcome::BadStatus(404));
    }

    #[tokio::test]
    async fn connection_failures_are_classified() {
        let updates = collect(input(vec![draft("x", "http://127.0.0.1:1/".to_string())])).await;
        assert!(matches!(
            results(&updates)[0].outcome,
            Outcome::Error {
                kind: FailureKind::Connection,
                ..
            }
        ));
    }

    /// A collection whose `dev` environment declares `TOKEN` and `NEW_TOKEN` as secrets.
    fn collection_with_secret(base: &str) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        std::fs::write(
            dir.path().join("environments/dev.yaml"),
            format!("vars: {{ BASE_URL: '{base}' }}\nsecrets: [TOKEN, NEW_TOKEN]\n"),
        )
        .unwrap();
        let secrets = collection.secrets_path("dev");
        std::fs::create_dir_all(secrets.parent().unwrap()).unwrap();
        std::fs::write(&secrets, "TOKEN=hunter2\n").unwrap();
        return dir;
    }

    fn in_collection(dir: &tempfile::TempDir, targets: Vec<Target>) -> RunInput {
        return RunInput {
            root: Some(dir.path().canonicalize().unwrap()),
            env: Some("dev".to_string()),
            use_cookies: true,
            targets,
            ..RunInput::default()
        };
    }

    #[tokio::test]
    async fn secrets_are_sent_but_never_shown() {
        let server = MockServer::start().await;
        Mock::given(header("x-token", "hunter2"))
            .respond_with(ResponseTemplate::new(200).set_body_string("echo hunter2"))
            .mount(&server)
            .await;
        let dir = collection_with_secret(&server.uri());
        let mut file = RequestFile::default();
        file.http.url = "${BASE_URL}/s?t=${TOKEN}".to_string();
        file.http.headers.push("x-token", Some("${TOKEN}".to_string()));
        let run_input = in_collection(&dir, vec![draft_file("s", file)]);

        let shown = preview(&run_input).unwrap();
        assert!(!shown.head.contains("hunter2"), "{}", shown.head);
        assert!(!shown.url.contains("hunter2"), "{}", shown.url);

        let updates = collect(run_input).await;
        for update in &updates {
            if let Update::Prepared { sent, .. } = update {
                assert!(!format!("{sent:?}").contains("hunter2"), "{sent:?}");
            }
        }
        let done = results(&updates);
        assert!(done[0].outcome.is_pass(), "{:?}", done[0].outcome);
        // The bytes stay raw for Save. What's displayed goes through the redactor.
        assert_eq!(done[0].redactor.redact("echo hunter2"), "echo ***");
        assert!(!done[0].json.as_ref().unwrap().to_string().contains("hunter2"));
    }

    #[tokio::test]
    async fn captures_reach_the_next_request_and_are_redacted() {
        let server = MockServer::start().await;
        Mock::given(path("/login"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"t":"sekret"}"#))
            .mount(&server)
            .await;
        Mock::given(path("/me"))
            .and(header("x-auth", "sekret"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let dir = collection_with_secret(&server.uri());

        let mut login = RequestFile::default();
        login.http.url = "${BASE_URL}/login".to_string();
        login.capture.insert("NEW_TOKEN".to_string(), "$.t".to_string());
        let mut me = RequestFile::default();
        me.http.url = "${BASE_URL}/me".to_string();
        me.http.headers.push("x-auth", Some("${NEW_TOKEN}".to_string()));
        me.assertions.status = vec![200];

        let run_input = in_collection(&dir, vec![draft_file("login", login), draft_file("me", me)]);
        let updates = collect(run_input).await;
        let done = results(&updates);
        assert_eq!(done.len(), 2);
        assert!(done[1].outcome.is_pass(), "{:?}", done[1].outcome);
        assert_eq!(done[0].captured, vec!["NEW_TOKEN"]);
        assert_eq!(done[0].redactor.redact("sekret"), "***");
    }

    #[tokio::test]
    async fn event_streams_arrive_live_and_others_dont() {
        let server = MockServer::start().await;
        Mock::given(path("/plain"))
            .respond_with(ResponseTemplate::new(200).set_body_string("hello"))
            .mount(&server)
            .await;
        Mock::given(path("/sse"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("data: 1\n\n", "text/event-stream"))
            .mount(&server)
            .await;
        let chunks = |updates: &[Update]| {
            updates.iter().filter(|u| matches!(u, Update::Chunk { .. })).count()
        };
        let plain = collect(input(vec![draft("p", format!("{}/plain", server.uri()))])).await;
        assert_eq!(chunks(&plain), 0);
        let sse = collect(input(vec![draft("s", format!("{}/sse", server.uri()))])).await;
        assert!(chunks(&sse) > 0);
    }

    #[tokio::test]
    async fn aborting_cancels_a_request_still_waiting_for_its_head() {
        let server = MockServer::start().await;
        Mock::given(path("/slow"))
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(30)))
            .mount(&server)
            .await;
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let task = tokio::spawn(run(
            input(vec![draft("slow", format!("{}/slow", server.uri()))]),
            sender,
        ));
        loop {
            if let Some(Update::Prepared { .. }) = receiver.recv().await {
                break;
            }
        }
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
    }

    #[tokio::test]
    async fn concurrent_runs_share_one_jar() {
        let dir = collection_with_secret("http://x");
        let run_input = in_collection(&dir, Vec::new());
        let first = open_session(&run_input).unwrap();
        let second = open_session(&run_input).unwrap();
        assert!(Arc::ptr_eq(&first.jar().unwrap(), &second.jar().unwrap()));
    }

    #[tokio::test]
    async fn a_saved_request_cant_read_outside_its_collection() {
        let server = MockServer::start().await;
        let dir = collection_with_secret(&server.uri());
        let root = dir.path().canonicalize().unwrap();
        let outside = tempfile::NamedTempFile::new().unwrap();
        let path = root.join("up.request.yaml");
        std::fs::write(
            &path,
            format!(
                "http:\n  url: ${{BASE_URL}}/up\n  body:\n    file: {}\n",
                outside.path().display()
            ),
        )
        .unwrap();
        let target = file_target(&path, Some(&root));
        let updates = collect(in_collection(&dir, vec![target])).await;
        let message = error_message(results(&updates)[0]);
        assert!(message.contains("outside the collection"), "{message}");
    }
}

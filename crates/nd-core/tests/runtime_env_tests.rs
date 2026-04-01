use nd_core::env::RuntimeEnv;
use nd_core::error::Error;

#[test]
fn merge_env_file_loads_values_and_overrides_duplicates() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), "# comment\nFOO=1\nBAR=\"x y\"\nFOO=2\n").unwrap();

    let env = RuntimeEnv::new();
    env.merge_env_file(tmp.path()).unwrap();

    assert_eq!(env.get("FOO").as_deref(), Some("2"));
    assert_eq!(env.get("BAR").as_deref(), Some("x y"));
}

#[test]
fn with_persistence_loads_existing_values_from_json_file() {
    let dir = tempfile::tempdir().unwrap();
    let persist_path = dir.path().join("runtime.json");
    std::fs::write(
        &persist_path,
        r#"{
  "TOKEN": "abc123",
  "COUNT": 7,
  "FLAGS": { "beta": true }
}"#,
    )
    .unwrap();

    let env = RuntimeEnv::new()
        .with_persistence(&Some(persist_path.clone()))
        .unwrap();

    assert_eq!(env.get("TOKEN").as_deref(), Some("abc123"));
    assert_eq!(env.get("COUNT").as_deref(), Some("7"));
    assert_eq!(env.get("FLAGS").as_deref(), Some(r#"{"beta":true}"#));
}

#[test]
fn persist_updates_runtime_and_writes_json_file() {
    let dir = tempfile::tempdir().unwrap();
    let persist_path = dir.path().join("runtime.json");

    let env = RuntimeEnv::new()
        .with_persistence(&Some(persist_path.clone()))
        .unwrap();
    env.persist("TOKEN", "fresh-value").unwrap();

    assert_eq!(env.get("TOKEN").as_deref(), Some("fresh-value"));

    let written = std::fs::read_to_string(&persist_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&written).unwrap();
    assert_eq!(json["TOKEN"], "fresh-value");
}

#[test]
fn persist_without_configured_file_returns_error() {
    let env = RuntimeEnv::new();
    let err = env.persist("TOKEN", "missing-file").unwrap_err();

    match err {
        Error::NoRuntimePersistFile { message } => {
            assert!(message.contains("TOKEN"));
        }
        other => panic!("unexpected error: {other}"),
    }
}

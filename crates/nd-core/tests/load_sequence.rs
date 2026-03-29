//! Tests for [`nd_core::load_sequence_file`] and empty-step validation via [`nd_core::execute_sequence`].

use nd_core::{execute_sequence, load_sequence_file, RunOptions};
use tempfile::tempdir;

#[test]
fn load_sequence_yaml() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("seq.yaml");
    std::fs::write(
        &p,
        b"version: 1\nsteps:\n  - file: a.yaml\n  - file: b.yaml\n",
    )
    .unwrap();
    let (s, base) = load_sequence_file(&p).unwrap();
    assert_eq!(s.steps.len(), 2);
    assert_eq!(s.name, None);
    assert_eq!(s.steps[0].file, "a.yaml");
    assert_eq!(base, dir.path());
}

#[test]
fn load_sequence_json_with_name() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("seq.json");
    std::fs::write(
        &p,
        r#"{"version":"1","name":"Smoke tests","steps":[{"file":"a.yaml"}]}"#,
    )
    .unwrap();
    let (s, _) = load_sequence_file(&p).unwrap();
    assert_eq!(s.name.as_deref(), Some("Smoke tests"));
    assert_eq!(s.steps.len(), 1);
}

#[test]
fn load_sequence_unsupported_extension_errors() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("seq.txt");
    std::fs::write(&p, b"{}").unwrap();
    assert!(load_sequence_file(&p).is_err());
}

#[tokio::test]
async fn execute_sequence_missing_step_file_errors() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("seq.yaml");
    std::fs::write(&p, b"version: 1\nsteps:\n  - file: does-not-exist.yaml\n").unwrap();
    let err = execute_sequence(&p, &RunOptions::default())
        .await
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("not found") || err.contains("SequenceStepNotFound"),
        "{err}"
    );
}

#[tokio::test]
async fn execute_sequence_rejects_empty_steps() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("empty.yaml");
    std::fs::write(&p, b"version: 1\nsteps: []\n").unwrap();
    let err = execute_sequence(&p, &RunOptions::default())
        .await
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("at least one step") || err.contains("sequence"),
        "{err}"
    );
}

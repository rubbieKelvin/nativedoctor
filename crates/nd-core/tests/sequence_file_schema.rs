//! JSON Schema export for [`nd_core::SequenceFile`].

use nd_core::sequence_file_json_schema;

#[test]
fn sequence_file_schema_serializes_to_json() {
    let v = sequence_file_json_schema();
    serde_json::to_string(&v).expect("schema value serializes");
}

#[test]
fn sequence_file_schema_contains_expected_keys() {
    let v = sequence_file_json_schema();
    let s = v.to_string();
    assert!(
        s.contains("SequenceFile") || s.contains("sequence"),
        "schema should describe sequence document: {s}"
    );
    assert!(s.contains("steps"), "schema should include steps: {s}");
    assert!(s.contains("path"), "schema should include step path: {s}");
}

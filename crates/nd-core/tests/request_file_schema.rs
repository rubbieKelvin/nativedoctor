//! JSON Schema export for [`nd_core::RequestFile`].

use nd_core::request_file_json_schema;

#[test]
fn request_file_schema_serializes_to_json() {
    let v = request_file_json_schema();
    serde_json::to_string(&v).expect("schema value serializes");
}

#[test]
fn request_file_schema_contains_expected_keys() {
    let v = request_file_json_schema();
    let s = v.to_string();
    assert!(
        s.contains("RequestFile") || s.contains("request"),
        "schema should describe request document: {s}"
    );
    assert!(s.contains("tags"), "schema should include tags: {s}");
    assert!(s.contains("summary"), "schema should include summary: {s}");
    assert!(
        s.contains("method") && s.contains("url"),
        "schema should include method and url: {s}"
    );
}

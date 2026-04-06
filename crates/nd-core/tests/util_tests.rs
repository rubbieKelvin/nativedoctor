use base64::Engine;
use nd_core::rhai::utils::{dynamic_to_json, json_to_dynamic};
use rhai::Dynamic;
use serde_json::{json, Value};

#[test]
fn round_trip_primitives() {
    let samples = vec![
        json!(null),
        json!(true),
        json!(false),
        json!(42),
        json!(-1),
        json!(3.25),
        json!("hello"),
        json!([]),
        json!([1, 2, 3]),
        json!({}),
        json!({"a": 1, "b": "x"}),
    ];
    for v in samples {
        let d = json_to_dynamic(&v);
        let back = dynamic_to_json(&d);
        assert_eq!(v, back, "round-trip failed for {v:?}");
    }
}

#[test]
fn bool_round_trip() {
    let d = Dynamic::from(true);
    assert_eq!(dynamic_to_json(&d), Value::Bool(true));
}

#[test]
fn blob_becomes_base64_string() {
    let d = Dynamic::from_blob(vec![1, 2, 3]);
    let v = dynamic_to_json(&d);
    assert_eq!(
        v,
        Value::String(base64::engine::general_purpose::STANDARD.encode([1u8, 2, 3]))
    );
}

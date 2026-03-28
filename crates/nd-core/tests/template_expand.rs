//! Integration tests for [`nd_core::expand_string`] and [`nd_core::expand_json_value`].

use nd_core::{expand_json_value, expand_string, RuntimeEnv};
use serde_json::json;

#[test]
fn expand_string_replaces_var() {
    let env = RuntimeEnv::from_process_env();
    env.set_runtime("FOO", "bar");
    assert_eq!(expand_string(&env, "${FOO}").unwrap(), "bar");
}

#[test]
fn expand_string_multiple_occurrences() {
    let env = RuntimeEnv::from_process_env();
    env.set_runtime("X", "1");
    assert_eq!(expand_string(&env, "${X}-${X}").unwrap(), "1-1");
}

#[test]
fn expand_string_missing_var_errors() {
    let env = RuntimeEnv::from_process_env();
    let err = expand_string(&env, "${__ND_CORE_MISSING_VAR_XYZ__}")
        .unwrap_err()
        .to_string();
    assert!(err.contains("__ND_CORE_MISSING_VAR_XYZ__"));
}

#[test]
fn expand_string_empty_when_no_placeholders() {
    let env = RuntimeEnv::from_process_env();
    assert_eq!(
        expand_string(&env, "no vars here").unwrap(),
        "no vars here"
    );
}

#[test]
fn expand_json_value_nested_strings_and_arrays() {
    let env = RuntimeEnv::from_process_env();
    env.set_runtime("X", "1");
    let v = json!({ "a": "${X}", "b": [ "${X}" ] });
    let e = expand_json_value(&env, &v).unwrap();
    assert_eq!(e, json!({ "a": "1", "b": ["1"] }));
}

#[test]
fn expand_json_value_object_keys() {
    let env = RuntimeEnv::from_process_env();
    env.set_runtime("K", "key");
    let v = json!({ "${K}": "v" });
    let e = expand_json_value(&env, &v).unwrap();
    assert_eq!(e, json!({ "key": "v" }));
}

#[test]
fn expand_json_value_leaves_numbers_bool_null() {
    let env = RuntimeEnv::from_process_env();
    let v = json!({ "n": 42, "b": true, "z": null });
    let e = expand_json_value(&env, &v).unwrap();
    assert_eq!(e, v);
}

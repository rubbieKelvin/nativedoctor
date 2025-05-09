use crate::parser::builder::Runtime;

#[test]
fn test_simple_request_call() {
    let runtime = Runtime::new("src/tests/test_yaml_files/simple_request_call.yaml", None).unwrap();
    assert!(runtime.schema.requests.contains_key("Ping"));
}

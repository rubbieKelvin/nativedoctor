use crate::parser::runner::Runner;

fn loaded_runner() -> Runner {
    return Runner::new(
        "src/tests/test_yaml_files/call_sequence_generation_check.api.yaml",
        None,
    )
    .unwrap();
}

#[test]
fn test_generate_call_stack_no_deps() {
    let runtime = loaded_runner();

    // Test RequestD (no dependencies)
    let queue = runtime
        .generate_call_stack("RequestD")
        .expect("Failed to generate queue for RequestD");

    assert_eq!(queue, vec!["RequestD"]);
}

#[test]
fn test_generate_call_stack_with_deps() {
    let runtime = loaded_runner();

    let queue = runtime
        .generate_call_stack("RequestA")
        .expect("Failed to generate queue for RequestA");

    assert_eq!(queue, vec!["RequestD", "RequestB", "RequestC", "RequestA"])
}

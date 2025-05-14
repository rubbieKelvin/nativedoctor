use crate::executor::runner::{Runner, ScriptEngine};

fn loaded_runner(complex: bool) -> Runner {
    return Runner::new(
        if complex {
            "src/tests/test_yaml_files/call_sequence_generation_check_complex.api.yaml"
        } else {
            "src/tests/test_yaml_files/call_sequence_generation_check.api.yaml"
        },
        None,
        ScriptEngine::None,
        false,
    )
    .unwrap();
}

#[test]
fn test_generate_call_stack_no_deps() {
    let runtime = loaded_runner(false);

    // Test RequestD (no dependencies)
    let queue = runtime
        .generate_call_queue("RequestD")
        .expect("Failed to generate queue for RequestD");

    assert_eq!(queue, vec!["RequestD"]);
}

#[test]
fn test_generate_call_stack_with_deps() {
    let runtime = loaded_runner(false);

    let queue = runtime
        .generate_call_queue("RequestA")
        .expect("Failed to generate queue for RequestA");

    assert_eq!(queue, vec!["RequestD", "RequestB", "RequestC", "RequestA"])
}

#[test]
#[should_panic(expected = "Circular dependency detected in request stack")]
fn test_complex_dep_gen() {
    let runtime = loaded_runner(true);
    runtime
        .generate_call_queue("RequestA")
        .expect("Failed to generate queue for RequestA");
}

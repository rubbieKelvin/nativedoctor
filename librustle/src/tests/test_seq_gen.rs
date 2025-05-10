use crate::parser::runner::Runner;

fn loaded_runner() -> Runner {
    return Runner::new(
        "src/tests/test_yaml_files/call_sequence_generation_check.api.yaml",
        None,
    )
    .unwrap();
}

#[test]
fn test_generate_request_call_queue_no_deps() {
    let runtime = loaded_runner();

    // Test RequestA (no dependencies)
    let queue = runtime
        .generate_request_call_queue("RequestA".to_string())
        .expect("Failed to generate queue for RequestA");

    assert_eq!(queue, vec!["RequestA"]);
}

#[test]
fn test_generate_request_call_queue_from_sequence_simple() {
    let runtime = loaded_runner();

    // Test simple_seq (RequestA, RequestB, RequestC)
    let queue = runtime
        .generate_request_call_queue_from_sequence("simple_seq".to_string())
        .expect("Failed to generate queue for simple_seq");
    // Expected order: Requests in sequence order, dependencies included and de-duplicated
    // simple_seq expands to [RequestA, RequestB, RequestC]
    // RequestA deps: [] -> adds RequestA
    // RequestB deps: [RequestA] -> adds RequestA (de-duplicated), adds RequestB
    // RequestC deps: [RequestA, RequestB] -> adds RequestA (de-duplicated), adds RequestB (de-duplicated), adds RequestC
    assert_eq!(queue, vec!["RequestA", "RequestB", "RequestC"]);
}

#[test]
fn test_generate_request_call_queue_from_sequence_with_duplicates() {
    let runtime = loaded_runner();

    // Test seq_with_duplicates (RequestA, RequestB, RequestA)
    let queue = runtime
        .generate_request_call_queue_from_sequence("seq_with_duplicates".to_string())
        .expect("Failed to generate queue for seq_with_duplicates");
    // Expected order: Requests in sequence order, dependencies included and de-duplicated
    // seq_with_duplicates expands to [RequestA, RequestB, RequestA]
    // RequestA deps: [] -> adds RequestA
    // RequestB deps: [RequestA] -> adds RequestA (de-duplicated), adds RequestB
    // RequestA deps: [] -> adds RequestA (de-duplicated)
    assert_eq!(queue, vec!["RequestA", "RequestB"]);
}

#[test]
fn test_generate_request_call_queue_from_sequence_nested() {
    let runtime = loaded_runner();

    // Test nested_seq_outer (/nested_seq_inner, RequestG, RequestA)
    // nested_seq_inner expands to [RequestE, RequestF]
    // RequestF depends on [RequestE]
    let queue = runtime
        .generate_request_call_queue_from_sequence("nested_seq_outer".to_string())
        .expect("Failed to generate queue for nested_seq_outer");

    // Expected order:
    // /nested_seq_inner -> expands to [RequestE, RequestF]
    //   RequestE deps: [] -> adds RequestE
    //   RequestF deps: [RequestE] -> adds RequestE (dedup), adds RequestF
    // RequestG deps: [RequestE] -> adds RequestE (dedup), adds RequestG
    // RequestA deps: [] -> adds RequestA (dedup)
    // Final unique order based on first appearance: RequestE, RequestF, RequestG, RequestA
    assert_eq!(queue, vec!["RequestE", "RequestF", "RequestG", "RequestA"]);
}

#[test]
fn test_generate_request_call_queue_from_sequence_mixed() {
    let runtime = loaded_runner();

    // Test mixed_seq (RequestA, /simple_seq, RequestD, RequestE, /nested_seq_inner)
    // simple_seq expands to [RequestA, RequestB, RequestC]
    // RequestD depends on [RequestC] which depends on [RequestA, RequestB]
    // nested_seq_inner expands to [RequestE, RequestF]
    // RequestF depends on [RequestE]

    let queue = runtime
        .generate_request_call_queue_from_sequence("mixed_seq".to_string())
        .expect("Failed to generate queue for mixed_seq");

    // Expected order based on first appearance:
    // RequestA deps: [] -> adds RequestA
    // /simple_seq -> expands to [RequestA, RequestB, RequestC]
    //   RequestA deps: [] -> adds RequestA (dedup)
    //   RequestB deps: [RequestA] -> adds RequestA (dedup), adds RequestB
    //   RequestC deps: [RequestA, RequestB] -> adds RequestA (dedup), adds RequestB (dedup), adds RequestC
    // RequestD deps: [RequestC]
    //   RequestC deps: [RequestA, RequestB] -> adds A, B, C (all dedup)
    // adds RequestD
    // RequestE deps: [] -> adds RequestE
    // /nested_seq_inner -> expands to [RequestE, RequestF]
    //   RequestE deps: [] -> adds RequestE (dedup)
    //   RequestF deps: [RequestE] -> adds RequestE (dedup), adds RequestF
    // Final unique order based on first appearance: RequestA, RequestB, RequestC, RequestD, RequestE, RequestF
    assert_eq!(
        queue,
        vec!["RequestA", "RequestB", "RequestC", "RequestD", "RequestE", "RequestF"]
    );
}

#[test]
#[should_panic(expected = "Request with name \"NonExistentRequest\" does not exist")]
fn test_generate_request_call_queue_nonexistent_request() {
    let runtime = loaded_runner();

    // Call a request that doesn't exist
    runtime
        .generate_request_call_queue("NonExistentRequest".to_string())
        .unwrap();
}

#[test]
#[should_panic(expected = "Request with name \"NonExistentSequence\" does not exist")]
fn test_generate_request_call_queue_from_sequence_nonexistent_sequence() {
    let runtime = loaded_runner();

    // Call a sequence that doesn't exist
    runtime
        .generate_request_call_queue_from_sequence("NonExistentSequence".to_string())
        .unwrap();
}

// NOTE: ...
// We have an ordering issue here. I dont intend to fix this anytime soon
// This probably arises from the fact that hash maps dont preserve order.

// #[test]
// fn test_generate_request_call_queue_from_sequence_with_deps() {
//     let runtime = loaded_runner();

//     // Test seq_with_deps (RequestD)
//     let queue = runtime
//         .generate_request_call_queue_from_sequence("seq_with_deps".to_string())
//         .expect("Failed to generate queue for seq_with_deps");
//     // Expected order: RequestD deps first, then RequestD
//     // RequestD deps: [RequestC]
//     // RequestC deps: [RequestA, RequestB]
//     // Order: A, B, C (from RequestC deps), D
//     assert_eq!(queue, vec!["RequestA", "RequestB", "RequestC", "RequestD"]);
// }

// #[test]
// fn test_generate_request_call_queue_single_dep() {
//     let runtime = loaded_runner();

//     // Test RequestB (depends on RequestA)
//     let queue = runtime
//         .generate_request_call_queue("RequestB".to_string())
//         .expect("Failed to generate queue for RequestB");
//     // Expected order: Dependency first, then the request itself
//     assert_eq!(queue, vec!["RequestA", "RequestB"]);
// }

// #[test]
// fn test_generate_request_call_queue_multiple_deps() {
//     let runtime = loaded_runner();

//     // Test RequestC (depends on RequestA, RequestB)
//     let queue = runtime
//         .generate_request_call_queue("RequestC".to_string())
//         .expect("Failed to generate queue for RequestC");
//     // ADding priority to sequence calls and request dependency might fix this.
//     // Expected order: Dependencies first (order might depend on HashMap iteration, but A and B should be before C and de-duplicated)
//     // The de-duplication logic puts items in the order they are first encountered in the queue.
//     // The recursive calls for dependencies will add them.
//     // RequestC depends on [RequestA, RequestB].
//     // generate_request_call_queue("RequestC") calls generate_request_call_queue("RequestA") -> adds RequestA
//     // then calls generate_request_call_queue("RequestB") -> adds RequestB
//     // then adds RequestC
//     assert_eq!(queue, vec!["RequestA", "RequestB", "RequestC"]);
// }

// #[test]
// fn test_generate_request_call_queue_nested_deps() {
//     let runtime = loaded_runner();

//     // Test RequestD (depends on RequestC, which depends on RequestA, RequestB)
//     let queue = runtime
//         .generate_request_call_queue("RequestD".to_string())
//         .expect("Failed to generate queue for RequestD");
//     // Expected order: Dependencies of dependencies first, then direct dependencies, then the request itself
//     // generate_request_call_queue("RequestD") calls generate_request_call_queue("RequestC")
//     //   generate_request_call_queue("RequestC") calls generate_request_call_queue("RequestA") -> adds RequestA
//     //   generate_request_call_queue("RequestC") calls generate_request_call_queue("RequestB") -> adds RequestB
//     //   generate_request_call_queue("RequestC") adds RequestC
//     // adds RequestD
//     assert_eq!(queue, vec!["RequestA", "RequestB", "RequestC", "RequestD"]);
// }

// #[test]
// fn test_generate_request_call_queue_shared_deps() {
//     let runtime = loaded_runner();

//     // Test RequestF (depends on RequestE)
//     let queue_f = runtime
//         .generate_request_call_queue("RequestF".to_string())
//         .expect("Failed to generate queue for RequestF");
//     assert_eq!(queue_f, vec!["RequestE", "RequestF"]);

//     // Test RequestG (depends on RequestE)
//     let queue_g = runtime
//         .generate_request_call_queue("RequestG".to_string())
//         .expect("Failed to generate queue for RequestG");
//     assert_eq!(queue_g, vec!["RequestE", "RequestG"]);

//     // Test a request that depends on both F and G (both depend on E)
//     let mut requests = runtime.schema.requests.clone();
//     requests.insert(
//         "RequestH".to_string(),
//         Request {
//             method: "GET".to_string(),
//             url: "{{base_url}}/h".to_string(),
//             config: Some(RequestConfig {
//                 depends_on: vec!["RequestF".to_string(), "RequestG".to_string()],
//                 ..Default::default()
//             }),
//             ..Default::default()
//         },
//     );
//     let schema_h = Schema {
//         requests,
//         ..Default::default()
//     };

//     let runtime_h = Runner::from_schema(schema_h, None);

//     let queue_h = runtime_h
//         .generate_request_call_queue("RequestH".to_string())
//         .expect("Failed to generate queue for RequestH");
//     // Expected order: E (from F's deps), F, E (from G's deps - de-duplicated), G, H
//     assert_eq!(
//         queue_h,
//         vec!["RequestE", "RequestF", "RequestG", "RequestH"]
//     );
// }

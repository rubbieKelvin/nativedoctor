use std::collections::HashMap;

use reqwest::Client;
use serde_json::json;

use crate::executor::{
    runner::Runner,
    schema::{Request, RequestBody, RequestConfig, Schema},
};

#[tokio::test]
async fn test_simple_request_call() {
    let client = Client::new();
    let runtime = Runner::new(
        "src/tests/test_yaml_files/simple_request_call.api.yaml",
        None,
    )
    .unwrap();
    assert!(runtime.schema.requests.contains_key("Ping"));
    let res = runtime
        .call_request("Ping".to_string(), &client)
        .await
        .unwrap();
    dbg!(res);
}

#[tokio::test]
async fn test_simple_get_request() {
    let mut requests = HashMap::<String, Request>::new();
    requests.insert(
        "Ping".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/200".to_string(), // Use a public test endpoint
            ..Default::default()
        },
    );

    let schema = Schema {
        requests,
        ..Default::default()
    };

    let client = Client::new();
    let runtime = Runner::from_schema(schema, None);

    let response = runtime
        .call_request("Ping".to_string(), &client)
        .await
        .expect("Failed to call Ping request");

    println!("Ping Response: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn test_post_request_with_json_body() {
    // 1. Define the schema programmatically
    let mut requests = HashMap::new();
    requests.insert(
        "PostJson".to_string(),
        Request {
            method: "POST".to_string(),
            url: "http://httpbin.org/anything".to_string(), // httpbin.org/anything echoes the request
            body: Some(RequestBody::Json {
                content: serde_yaml::to_value(json!({
                    "name": "Test Item",
                    "value": 123,
                    "is_active": true
                }))
                .unwrap(),
            }),
            headers: Some(HashMap::from([
                ("Content-Type".to_string(), "application/json".to_string()),
                ("X-Test-Header".to_string(), "test_value".to_string()),
            ])),
            ..Default::default()
        },
    );

    let schema = Schema {
        requests,
        ..Default::default()
    };

    // 2. Create the runtime
    let client = Client::new();
    let runtime = Runner::from_schema(schema, None);

    // 3. Call the request
    let response = runtime
        .call_request("PostJson".to_string(), &client)
        .await
        .expect("Failed to call PostJson request");

    // 4. Assert the outcome
    println!("PostJson Response Status: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200);

    // Assert the echoed request body and headers from httpbin.org/anything
    let echoed_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body as JSON");
    println!("Echoed Response Body: {:#?}", echoed_response);

    // Assert the JSON body was sent correctly
    assert_eq!(echoed_response["json"]["name"], "Test Item");
    assert_eq!(echoed_response["json"]["value"], 123);
    assert_eq!(echoed_response["json"]["is_active"], true);

    // Assert headers were sent correctly (httpbin adds its own headers too)
    // let headers =  echoed_response["headers"]
    //     .as_mapping()
    //     .expect("Headers should be a map");
    // assert_eq!(
    //     headers
    //         .get(&Value::String("X-Test-Header".to_string()))
    //         .unwrap(),
    //     &Value::String("test_value".to_string())
    // );
    // assert_eq!(
    //     headers
    //         .get(&Value::String("Content-Type".to_string()))
    //         .unwrap(),
    //     &Value::String("application/json".to_string())
    // );
}

#[tokio::test]
async fn test_request_with_dependencies() {
    // 1. Define the schema programmatically with dependencies
    let mut requests = HashMap::new();

    requests.insert(
        "Dependency1".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/201".to_string(), // Dependency 1 returns 201
            ..Default::default()
        },
    );

    requests.insert(
        "Dependency2".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/202".to_string(), // Dependency 2 returns 202
            ..Default::default()
        },
    );

    requests.insert(
        "MainRequest".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/200".to_string(), // Main request returns 200
            config: Some(RequestConfig {
                depends_on: vec!["Dependency1".to_string(), "Dependency2".to_string()],
                ..Default::default()
            }),
            ..Default::default()
        },
    );

    let schema = Schema {
        requests,
        ..Default::default()
    };

    // 2. Create the runtime
    let client = Client::new();
    let runtime = Runner::from_schema(schema, None);

    // 3. Call the main request (which should trigger dependencies)
    let response = runtime
        .call_request("MainRequest".to_string(), &client)
        .await
        .expect("Failed to call MainRequest");

    // 4. Assert the outcome
    println!("MainRequest Response Status: {:?}", response.status());
    assert!(response.status().is_success());
    assert_eq!(response.status().as_u16(), 200); // Assert MainRequest's status

    // TODO: i removed dependency_resposnse from the result of runtime.call_request so the assertations below might not work
    // Assert dependency responses are present and correct
    // assert_eq!(result.depenency_responses.len(), 2);
    // assert!(result.depenency_responses.contains_key("Dependency1"));
    // assert!(result.depenency_responses.contains_key("Dependency2"));

    // let dep1_result = result.depenency_responses.get("Dependency1").unwrap();
    // println!(
    //     "Dependency1 Response Status: {:?}",
    //     dep1_result.response.status()
    // );
    // assert!(dep1_result.response.status().is_success());
    // assert_eq!(dep1_result.response.status().as_u16(), 201); // Assert Dependency1's status
    // assert!(dep1_result.depenency_responses.is_empty()); // Dependencies of dependencies should be empty in this simple case

    // let dep2_result = result.depenency_responses.get("Dependency2").unwrap();
    // println!(
    //     "Dependency2 Response Status: {:?}",
    //     dep2_result.response.status()
    // );
    // assert!(dep2_result.response.status().is_success());
    // assert_eq!(dep2_result.response.status().as_u16(), 202); // Assert Dependency2's status
    // assert!(dep2_result.depenency_responses.is_empty()); // Dependencies of dependencies should be empty
}

#[tokio::test]
async fn test_call_sequence() {
    // 1. Define the schema programmatically with requests and a sequence
    let mut requests = HashMap::new();
    requests.insert(
        "Step1".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/200".to_string(),
            ..Default::default()
        },
    );
    requests.insert(
        "Step2".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/204".to_string(),
            ..Default::default()
        },
    );

    let mut calls = HashMap::new();
    calls.insert(
        "simple_sequence".to_string(),
        vec!["Step1".to_string(), "Step2".to_string()],
    );

    // let schema = Schema {
    //     requests,
    //     calls,
    //     ..Default::default()
    // };

    // 2. Create the runtime
    // let client = Client::new();
    // let runtime = Runner::from_schema(schema, None);

    // TODO: i removed support for call sequence
    // 3. Call the sequence
    // The call_sequence function currently returns Result<()>, so we can only assert success or failure
    // let result = runtime
    //     .call_sequence("simple_sequence".to_string(), &client)
    //     .await;

    // 4. Assert the outcome
    // assert!(
    //     result.is_ok(),
    //     "Sequence execution failed: {:?}",
    //     result.err()
    // );
}

#[tokio::test]
async fn test_nested_sequence() {
    // 1. Define the schema programmatically with requests and nested sequences
    let mut requests = HashMap::new();
    requests.insert(
        "InnerStep1".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/200".to_string(),
            ..Default::default()
        },
    );
    requests.insert(
        "InnerStep2".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/204".to_string(),
            ..Default::default()
        },
    );
    requests.insert(
        "OuterStep".to_string(),
        Request {
            method: "GET".to_string(),
            url: "http://httpbin.org/status/200".to_string(),
            ..Default::default()
        },
    );

    let mut calls = HashMap::new();
    calls.insert(
        "inner_sequence".to_string(),
        vec!["InnerStep1".to_string(), "InnerStep2".to_string()],
    );
    // A sequence that calls the inner sequence, then another step
    calls.insert(
        "outer_sequence".to_string(),
        vec!["/inner_sequence".to_string(), "OuterStep".to_string()],
    );

    // TODO: i removed support for call sequence
    // let schema = Schema {
    //     requests,
    //     calls,
    //     ..Default::default()
    // };

    // 2. Create the runtime
    // let client = Client::new();
    // let runtime = Runner::from_schema(schema, None);

    // 3. Call the outer sequence
    // let result = runtime
    //     .call_sequence("outer_sequence".to_string(), &client)
    //     .await;

    // 4. Assert the outcome
    // assert!(
    //     result.is_ok(),
    //     "Nested sequence execution failed: {:?}",
    //     result.err()
    // );
}

#[tokio::test]
async fn test_imports() {
    let file = "src/tests/test_yaml_files/test_imports/base.api.yaml";
    let runtime = Runner::new(file, None).unwrap();

    dbg!(file);
    dbg!(&runtime.schema.filename);

    // check that we set file name
    assert!(!runtime.schema.filename.is_empty());
    assert!(runtime.schema.filename.ends_with(file));
    assert!(runtime.schema.env.contains_key("email"));
    assert!(runtime.schema.env.contains_key("authId"));
    assert!(runtime.schema.env.contains_key("username"));
}

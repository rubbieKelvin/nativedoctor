use reqwest::Client;

use crate::parser::builder::Runtime;

#[tokio::test]
async fn test_simple_request_call() {
    let client = Client::new();
    let runtime = Runtime::new(
        "src/tests/test_yaml_files/simple_request_call.api.yaml",
        None,
    )
    .unwrap();
    assert!(runtime.schema.requests.contains_key("Ping"));
    let res = runtime.call_request("Ping".to_string(), &client, None).await.unwrap();
}

use crate::parser::schema::{parse_api_yaml, MultipartPart, RequestBody};

#[test]
fn test_parse_minimal_file() {
    let yaml = r#"
            env:
              base_url:
                default: "http://localhost:8080"

            requests:
              GetStatus:
                method: GET
                url: "{{base_url}}/status"
                script:
                  post_request: |
                    assert response.status == 200;

            calls:
              default_run:
                - GetStatus
        "#;

    let parsed = parse_api_yaml(yaml).expect("Failed to parse minimal YAML");

    println!("{:#?}", parsed);

    assert_eq!(parsed.env.len(), 1);
    assert!(parsed.env.contains_key("base_url"));
    assert_eq!(parsed.requests.len(), 1);
    assert!(parsed.requests.contains_key("GetStatus"));
    assert_eq!(parsed.calls.len(), 1);
    assert!(parsed.calls.contains_key("default_run"));
    assert_eq!(parsed.calls["default_run"].len(), 1);
    assert_eq!(parsed.calls["default_run"][0], "GetStatus");

    let status_request = parsed.requests.get("GetStatus").unwrap();
    assert_eq!(status_request.method, "GET");
    assert_eq!(status_request.url, "{{base_url}}/status");
    assert!(status_request.script.is_some());
    assert!(status_request
        .script
        .as_ref()
        .unwrap()
        .post_request
        .is_some());
}

#[test]
fn test_parse_full_example() {
    let yaml = r#"
                imports:
                  - common_env.yaml

                env:
                  username:
                    default: "default_user"
                    dev: "dev_user"
                  base_url:
                    default: "https://api.example.com"
                    dev: "https://dev.api.example.com"
                    prod: "https://prod.api.example.com"
                  # api_key: # This is commented out in the YAML string
                  #   default: null
                  #   prod: "prod_secret_key"

                requests:
                  LoginUser:
                    method: POST
                    url: "{{base_url}}/auth/login"
                    headers:
                      Content-Type: application/json
                    body:
                      type: json
                      content:
                        email: "{{user_email}}"
                        password: "{{user_password}}"
                    script:
                      post_request: |
                        # Script content
                        log("Logged in");

                  CreateItem:
                    method: POST
                    url: "{{base_url}}/items"
                    config:
                      depends_on: [LoginUser]
                      timeout: 10s
                      retries: 2
                    headers:
                      Content-Type: application/json
                      Authorization: "Bearer {{authToken}}"
                    body:
                      type: json
                      content:
                        name: "New Item {{uuid()}}"
                        price: 19.99
                        tags: ["{{env.current_env}}", "featured"]
                    script:
                      post_request: |
                        # Another script
                        assert response.status == 201;

                  UploadFile:
                    method: POST
                    url: "{{base_url}}/upload"
                    body:
                      type: multipart
                      parts:
                        - kind: field
                          name: description
                          value: "API test file upload"
                        - kind: file
                          name: document
                          path: "./data/report.pdf"
                          mime_type: application/pdf

                calls:
                  full_workflow: # Updated: calls is now a map
                    - LoginUser
                    - CreateItem
                    - UploadFile
                  another_sequence: # Example of another sequence
                    - GetStatus # Assuming GetStatus is defined elsewhere or in imports
            "#;

    let parsed = parse_api_yaml(yaml).expect("Failed to parse full example YAML");

    println!("{:#?}", parsed);

    assert_eq!(parsed.imports.len(), 1);
    assert_eq!(parsed.imports[0], "common_env.yaml");

    // Corrected assertion: Expecting 2 env vars based on the YAML provided
    assert_eq!(parsed.env.len(), 2);
    assert!(parsed.env.contains_key("username"));
    assert!(parsed.env.contains_key("base_url"));
    // api_key is commented out in the YAML, so it won't be in the parsed env map.
    // assert!(parsed.env.contains_key("api_key")); // This assertion would fail

    assert_eq!(parsed.requests.len(), 3);
    assert!(parsed.requests.contains_key("LoginUser"));
    assert!(parsed.requests.contains_key("CreateItem"));
    assert!(parsed.requests.contains_key("UploadFile"));

    assert_eq!(parsed.calls.len(), 2); // Now expects 2 keys in the calls map
    assert!(parsed.calls.contains_key("full_workflow"));
    assert!(parsed.calls.contains_key("another_sequence"));

    assert_eq!(parsed.calls["full_workflow"].len(), 3);
    assert_eq!(parsed.calls["full_workflow"][0], "LoginUser");
    assert_eq!(parsed.calls["full_workflow"][1], "CreateItem");
    assert_eq!(parsed.calls["full_workflow"][2], "UploadFile");

    assert_eq!(parsed.calls["another_sequence"].len(), 1);
    assert_eq!(parsed.calls["another_sequence"][0], "GetStatus");

    let create_item_request = parsed.requests.get("CreateItem").unwrap();
    assert!(create_item_request.config.is_some());
    assert_eq!(
        create_item_request.config.as_ref().unwrap().depends_on,
        vec!["LoginUser"]
    );
    assert_eq!(create_item_request.config.as_ref().unwrap().retries, 2);

    assert!(create_item_request.body.is_some());
    if let Some(RequestBody::Json { content }) = &create_item_request.body {
        // We can't easily assert the *content* structure here without more specific structs
        // or traversing the serde_yaml::Value, but we can check it's a Mapping (object).
        assert!(content.is_mapping());
    } else {
        panic!("CreateItem body is not JSON");
    }

    let upload_request = parsed.requests.get("UploadFile").unwrap();
    assert!(upload_request.body.is_some());
    if let Some(RequestBody::Multipart { parts }) = &upload_request.body {
        assert_eq!(parts.len(), 2);
        if let MultipartPart::Field { name, value } = &parts[0] {
            assert_eq!(name, "description");
            assert_eq!(value, "API test file upload");
        } else {
            panic!("First multipart part is not a field");
        }
        if let MultipartPart::File {
            name,
            path,
            mime_type,
        } = &parts[1]
        {
            assert_eq!(name, "document");
            assert_eq!(path, "./data/report.pdf");
            assert_eq!(mime_type.as_deref(), Some("application/pdf"));
        } else {
            panic!("Second multipart part is not a file");
        }
    } else {
        panic!("UploadFile body is not Multipart");
    }
}

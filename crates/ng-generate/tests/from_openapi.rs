//! Integration: OpenAPI fixture → request files → [`nd_core::load_request_file`].

use std::path::Path;

use nd_core::load_request_file;
use ng_generate::{generate_from_openapi_path, GenerateOptions, OutputFormat};
use tempfile::tempdir;

#[test]
fn generates_three_operations_and_loads_one() {
    let dir = tempdir().unwrap();
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixture_petstore_min.json");
    let out = dir.path().join("out");
    let report = generate_from_openapi_path(
        &fixture,
        &out,
        GenerateOptions {
            format: OutputFormat::Yaml,
        },
    )
    .expect("generate");

    assert_eq!(report.files_written.len(), 3);

    let list_pets = out.join("listpets.yaml");
    assert!(list_pets.is_file(), "expected listpets.yaml");
    let (req, _) = load_request_file(&list_pets).expect("load yaml");
    assert_eq!(req.request.method, "GET");
    assert_eq!(req.request.url, "https://api.example.com/pets");
    assert_eq!(req.name.as_deref(), Some("listPets"));
    assert_eq!(req.request.summary.as_deref(), Some("List pets"));

    let get_pet = out.join("getpet.yaml");
    let (req2, _) = load_request_file(&get_pet).expect("load getPet");
    assert_eq!(req2.request.url, "https://api.example.com/pets/${petId}");

    let note = out.join("createpetnote.yaml");
    let (req3, _) = load_request_file(&note).expect("load post");
    assert!(req3.request.body.is_some());
}

#[test]
fn openapi_3_1_rejected() {
    let dir = tempdir().unwrap();
    let p = dir.path().join("spec.json");
    std::fs::write(
        &p,
        r#"{"openapi":"3.1.0","info":{"title":"x","version":"1"},"paths":{}}"#,
    )
    .unwrap();
    let err = generate_from_openapi_path(&p, dir.path().join("out"), GenerateOptions::default())
        .expect_err("3.1 should fail");
    assert!(err.to_string().contains("3.1"), "unexpected error: {err}");
}

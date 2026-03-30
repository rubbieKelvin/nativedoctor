//! Writes JSON Schema files for request and sequence documents to `schema/` at the repo root.
//!
//! `cargo run -p nd-core --example export_schemas`

use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("nd-core should live at crates/nd-core");

    let schema_dir = repo_root
        .join("schema")
        .join(concat!("v", env!("CARGO_PKG_VERSION")));

    fs::create_dir_all(&schema_dir).expect("create schema/");

    let request = nd_core::request_file_json_schema();
    let sequence = nd_core::sequence_file_json_schema();

    // Write request json
    let filename = "request.schema.json";

    fs::write(
        schema_dir.join(filename),
        serde_json::to_string_pretty(&request).expect("request schema JSON"),
    )
    .expect(format!("write {filename}").as_str());

    // Write sequence json
    let filename = "sequence.schema.json";

    fs::write(
        schema_dir.join(filename),
        serde_json::to_string_pretty(&sequence).expect("sequence schema JSON"),
    )
    .expect(format!("write {filename}").as_str());

    // Write request yaml
    let filename = "request.schema.yaml";

    fs::write(
        schema_dir.join(filename),
        serde_yaml::to_string(&request).expect("request schema YAML"),
    )
    .expect(format!("write {filename}").as_str());

    // Write sequence yaml
    let filename = "sequence.schema.yaml";

    fs::write(
        schema_dir.join(filename),
        serde_yaml::to_string(&sequence).expect("sequence schema YAML"),
    )
    .expect(format!("write {filename}").as_str());

    eprintln!("Wrote schemas under {}", schema_dir.display());
}

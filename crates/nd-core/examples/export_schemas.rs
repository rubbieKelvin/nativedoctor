//! Writes JSON Schema files for request and sequence documents to `schema/` at the repo root.
//!
//! `cargo run -p nd-core --example export_schemas`

use nd_core::model::request::RequestFile;
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

    let request = RequestFile::schema();

    // Write request json
    let filename = "request.schema.json";

    fs::write(
        schema_dir.join(filename),
        serde_json::to_string_pretty(&request).expect("request schema JSON"),
    )
    .expect(format!("write {filename}").as_str());

    // Write request yaml
    let filename = "request.schema.yaml";

    fs::write(
        schema_dir.join(filename),
        serde_yaml::to_string(&request).expect("request schema YAML"),
    )
    .expect(format!("write {filename}").as_str());
}

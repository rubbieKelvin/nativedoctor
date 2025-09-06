use std::path::Path;

use anyhow::Context;
use reqwest::blocking;

use crate::{defs, schemas::request::RequestSchema, utils::generate_call_request_sequence};

pub fn run_native_doctor_path(path: &Path, no_deps: bool) -> Result<(), anyhow::Error> {
    let client = blocking::Client::new();

    return match defs::FileType::from_path(path) {
        Some(defs::FileType::RequestYamlFile) => {
            run_single_request_file(&client, path, no_deps)?;
            Ok(())
        }
        Some(defs::FileType::ProjectYamlFile) => {
            run_call_sequence(&client, path)?;
            Ok(())
        }
        _ => anyhow::bail!("File type not supported"),
    };
}

// Runs single request file and no extra dependency
fn _run_request_schema(
    client: &blocking::Client,
    schema: RequestSchema,
) -> Result<blocking::Response, anyhow::Error> {
    println!("Running request: {}", schema.name);
    let request = schema.build_blocking_reqwest(client)?;
    return request.send().context("Error sending request");
}

fn run_single_request_file(
    client: &blocking::Client,
    path: &Path,
    no_deps: bool,
) -> Result<(), anyhow::Error> {
    let schema = RequestSchema::read_from_path(path).context("Cannot run request file")?;
    if no_deps {
        _run_request_schema(client, schema)?;
    } else {
        let call_sequence = generate_call_request_sequence(schema, vec![])?;
        for schema in call_sequence {
            _run_request_schema(client, schema)?;
        }
    }
    return Ok(());
}

fn run_call_sequence(client: &blocking::Client, path: &Path) -> Result<(), anyhow::Error> {
    todo!("Implement call sequence");
    return Ok(());
}

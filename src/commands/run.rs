use std::path::Path;

use anyhow::Context;
use reqwest::blocking;

use crate::{defs, schemas::request::RequestSchema};

pub fn run_native_doctor_path(path: &Path) -> Result<(), anyhow::Error> {
    let client = blocking::Client::new();

    return match defs::FileType::from_path(path) {
        Some(defs::FileType::RequestYamlFile) => {
            run_single_request_file(&client, path)?;
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
fn run_single_request_file(
    client: &blocking::Client,
    path: &Path,
) -> Result<blocking::Response, anyhow::Error> {
    let schema = RequestSchema::read_from_path(path).context("Cannot run request file")?;
    let request = schema.build_blocking_reqwest(client)?;

    return request.send().context("Error sending request");
}

fn run_call_sequence(client: &blocking::Client, path: &Path) -> Result<(), anyhow::Error> {
    return Ok(());
}

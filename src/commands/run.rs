use std::path::Path;

use anyhow::Context;

use crate::{defs, schemas::request::RequestSchema};

pub fn run_native_doctor_path(path: &Path) -> Result<(), anyhow::Error> {
    return match defs::FileType::from_path(path) {
        Some(defs::FileType::RequestYamlFile) => run_request_file(path),
        _ => anyhow::bail!("File type not supported"),
    };
}

fn run_request_file(path: &Path) -> Result<(), anyhow::Error> {
    let schema = RequestSchema::read_from_path(path).context("Cannot run request file")?;
    return Ok(());
}

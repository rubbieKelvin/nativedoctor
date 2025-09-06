use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};
use reqwest::blocking;

use crate::{
    constants::EXTENSION_PROJECT_FILE_YAML,
    defs,
    schemas::{project::ProjectSchema, request::RequestSchema},
    utils::generate_call_request_sequence,
};

pub fn run_native_doctor_path(
    input: Option<String>,
    no_deps: bool,
    cwd: &Path,
) -> Result<(), anyhow::Error> {
    let client = blocking::Client::new();

    // if the input is a request yaml file, we need to run it
    if input.is_some()
        && matches!(
            defs::FileType::from_path(Path::new(&input.clone().unwrap())),
            Some(defs::FileType::ProjectYamlFile)
        )
    {
        let filename = &input.unwrap();
        let path = cwd.join(filename);
        let path = if path.is_absolute() {
            &path
        } else {
            let rough_path = &cwd.join(filename);
            &canonicalize(rough_path).context("Failed to canonicalize path")?
        };

        // validate path
        // path has to exist
        if !path.try_exists()? {
            bail!("No such file: {:?}", path)
        }

        // path has to be a file
        if !path.is_file() {
            bail!("Path is not a file")
        }

        run_single_request_file(&client, &path, no_deps)?;
    } else {
        // this is a call
        let project_path = cwd.join(format!("{}.{}", "main", EXTENSION_PROJECT_FILE_YAML));

        // check that the project path exists
        if !project_path.try_exists()? {
            bail!("No such file: {:?}", project_path)
        }

        // check that the project path is a file
        if !project_path.is_file() {
            bail!("Path is not a file")
        }

        run_call_sequence(&client, input.clone(), &project_path, no_deps)?;
    }
    return Ok(());
}

// Runs single request file and no extra dependency
fn _run_request_schema(
    client: &blocking::Client,
    schema: RequestSchema,
) -> Result<(), anyhow::Error> {
    println!("Running request: {}", schema.name);
    let request = schema.build_blocking_reqwest(client)?;
    let response = request.send().context("Error sending request")?;
    println!("Response: {:?}", response.status());
    return Ok(());
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

fn run_call_sequence(
    client: &blocking::Client,
    name: Option<String>,
    path: &Path,
    no_deps: bool,
) -> Result<(), anyhow::Error> {
    let project = ProjectSchema::read_from_path(path).context("Cannot read project file")?;
    let parent = path.parent().unwrap();

    let name = name.unwrap_or(project.default_sequence.unwrap_or("".to_string()));

    if name.is_empty() {
        bail!("No default sequence specified");
    }

    if let Some(sequence) = project.sequence
        && sequence.contains_key(&name)
    {
        let sequence = sequence
            .get(&name)
            .unwrap()
            .iter()
            .map(|file_str| parent.join(file_str))
            .collect::<Vec<PathBuf>>();

        for request_path in sequence {
            run_single_request_file(client, &request_path, no_deps)?;
        }
    } else {
        bail!("No such sequence \"{}\"", name);
    }

    return Ok(());
}

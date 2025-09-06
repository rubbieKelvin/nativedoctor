use std::{env, path::PathBuf};

use anyhow::Context;

use crate::schemas::request::RequestSchema;

/// Converts a string into a URL-friendly "slug".
pub fn slugify<S: AsRef<str>>(input: S) -> String {
    let mut slug = String::new();

    // prevents leading hyphen
    let mut last_was_hyphen = true;

    for c in input.as_ref().to_lowercase().chars() {
        if c.is_alphanumeric() {
            slug.push(c);
            last_was_hyphen = false;
        } else if !last_was_hyphen {
            slug.push('-');
            last_was_hyphen = true;
        }
    }

    // remove  trailing hyphen that may have been added at the end
    while slug.ends_with('-') {
        slug.pop();
    }

    return slug;
}

/// Gets the current directory
pub fn get_current_directory() -> Result<PathBuf, anyhow::Error> {
    env::current_dir().context("Could not access the current directory")
}

/// Recursively generates a call sequence for a request by first calling its dependencies.
///performs a depth-first traversal of the dependency graph. i'm using
/// the `trace` vector to keep track of the travasal path to detect and prevent cyclic dependencies.
pub fn generate_call_request_sequence(
    root_request: RequestSchema,
    trace: Vec<PathBuf>,
) -> Result<Vec<RequestSchema>, anyhow::Error> {
    // check for a cycle.
    //  if the current request's path is already in the trace, we've found a cycle.
    let current_path = root_request.path.clone().ok_or_else(|| {
        anyhow::anyhow!("Request with name '{}' is missing a path. All requests must have a path for call sequence generation.", root_request.name)
    })?;

    if trace.contains(&current_path) {
        return Err(anyhow::anyhow!(
            "Cyclic dependency detected involving '{}'.",
            current_path.display()
        ));
    }

    let mut new_trace = trace;
    new_trace.push(current_path);

    let dependencies = root_request.dependencies()?;

    // 4. Recursively generate the call sequence for all dependencies.
    let mut sequence = Vec::new();
    for dependency in dependencies {
        let dep_schema = dependency?;
        let dep_sequence = generate_call_request_sequence(dep_schema, new_trace.clone())?;
        sequence.extend(dep_sequence);
    }

    sequence.push(root_request);
    Ok(sequence)
}

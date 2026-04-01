//! `nativedoctor new`: write starter files by serializing [`nd_core`] schema types.
use std::path::PathBuf;

use nd_constants::urls::{PUBLIC_REQUEST_JSON_SCHEMA_URL, PUBLIC_REQUEST_YAML_SCHEMA_URL};
use nd_core::model::request::RequestFile;
use nd_core::model::with_root_schema_url;
use tracing::debug;

use crate::{Cli, Command};

#[derive(Debug, Clone)]
pub(crate) struct NewOption {
    url: Option<String>,
    name: Option<String>,
    path: PathBuf,
}

impl NewOption {
    pub(crate) fn from_cli(cli: &Cli) -> NewOption {
        return match &cli.command {
            Some(Command::New { url, name, path }) => NewOption {
                url: url.clone(),
                name: name.clone(),
                path: path.clone(),
            },
            _ => unreachable!("We should never have gotten here"),
        };
    }
}

pub fn run_new(option: NewOption) -> Result<(), String> {
    // path check
    if option.path.exists() {
        return Err(format!(
            "refusing to overwrite existing file: {}",
            option.path.display()
        ));
    }

    let ext = option
        .path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();

    // create and mutate document
    let mut doc = RequestFile::default();

    if let Some(name) = option.name {
        doc.name = Some(name);
    }

    if let Some(url) = option.url {
        doc.request.url = url;
    }

    // inject schema url
    let schema_url = match ext.as_str() {
        "json" => PUBLIC_REQUEST_JSON_SCHEMA_URL,
        "yaml" | "yml" => PUBLIC_REQUEST_YAML_SCHEMA_URL,
        _ => return Err(format!("unsupported extension for request: {ext}")),
    };

    let v = serde_json::to_value(&doc).map_err(|e| e.to_string())?;
    let v = with_root_schema_url(v, schema_url);

    // convert to string
    let content = match ext.as_str() {
        "json" => serde_json::to_string_pretty(&v).map_err(|e| e.to_string()),
        "yaml" | "yml" => serde_yaml::to_string(&v).map_err(|e| e.to_string()),
        _ => Err(format!("unsupported extension for request: {ext}")),
    }?;

    // write
    debug!(
        path = %option.path.display(),
        bytes = content.len(),
        "writing new template"
    );
    std::fs::write(option.path, content).map_err(|e| e.to_string())?;
    return Ok(());
}

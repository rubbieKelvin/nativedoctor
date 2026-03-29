//! Emit [`nd_core::RequestFile`] values to disk, with YAML quoting for `$` in URLs.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use nd_core::RequestFile;
use openapiv3::{OpenAPI, ReferenceOr};

use super::build_request::{file_stem, operation_to_request_file, unique_stem};
use crate::error::{Error, Result};

/// Serialization format for generated request definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Yaml,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "yaml" | "yml" => Ok(Self::Yaml),
            "json" => Ok(Self::Json),
            other => Err(format!("expected yaml or json, got {other}")),
        }
    }
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::Yaml => "yaml",
            Self::Json => "json",
        }
    }
}

/// YAML 1.1 treats `$` in plain scalars as alias syntax; quote `url:` values that contain `${...}`.
fn quote_yaml_urls_containing_dollar(text: &str) -> String {
    let re = regex::Regex::new(r"^(?P<prefix>\s+url:\s+)(?P<val>.+)$").expect("valid regex");
    let mut out = String::with_capacity(text.len() + 16);
    for line in text.lines() {
        if let Some(cap) = re.captures(line) {
            let val = cap.name("val").expect("val").as_str();
            if val.contains("${") && !(val.starts_with('"') || val.starts_with('\'')) {
                let prefix = cap.name("prefix").expect("prefix").as_str();
                let escaped = val.replace('\\', "\\\\").replace('"', "\\\"");
                out.push_str(prefix);
                out.push('"');
                out.push_str(&escaped);
                out.push('"');
                out.push('\n');
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Serializes `file` and writes it to `path`, creating parent directories when needed.
pub fn write_request_file(path: &Path, file: &RequestFile, format: OutputFormat) -> Result<()> {
    let data = match format {
        OutputFormat::Yaml => {
            let raw = serde_yaml::to_string(file).map_err(Error::Yaml)?;
            quote_yaml_urls_containing_dollar(&raw)
        }
        OutputFormat::Json => serde_json::to_string_pretty(file).map_err(Error::Json)?,
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| Error::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    std::fs::write(path, data).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(())
}

/// Generate one file per operation under `out_dir`.
pub fn write_all_operations(
    api: &OpenAPI,
    out_dir: &Path,
    format: OutputFormat,
) -> Result<Vec<PathBuf>> {
    std::fs::create_dir_all(out_dir).map_err(|source| Error::Io {
        path: out_dir.to_path_buf(),
        source,
    })?;

    let mut used_stems = HashSet::new();
    let mut written = Vec::new();

    for (path_str, path_ref) in api.paths.iter() {
        let path_item = match path_ref {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { reference } => {
                return Err(Error::PathItemRef(reference.clone()));
            }
        };

        for (method, operation) in path_item.iter() {
            let req = operation_to_request_file(api, path_str, method, operation, path_item)?;
            let stem = file_stem(operation, method, path_str);
            let unique = unique_stem(&stem, &mut used_stems);
            let out_path = out_dir.join(format!("{unique}.{}", format.extension()));
            write_request_file(&out_path, &req, format)?;
            written.push(out_path);
        }
    }

    Ok(written)
}

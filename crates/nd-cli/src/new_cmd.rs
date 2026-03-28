//! `nativedoctor new`: write starter files by serializing [`nd_core`] schema types.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nd_core::{HttpRequestSpec, RequestFile, SequenceFile, SequenceStep};

/// Build the default sequence document (same fields as the former template).
fn default_sequence_file() -> SequenceFile {
    SequenceFile {
        version: 1,
        name: Some("Sample flow".into()),
        steps: vec![SequenceStep {
            file: "example-request.yaml".into(),
        }],
    }
}

/// Build the default request document (same fields as the former template).
fn default_request_file() -> RequestFile {
    RequestFile {
        version: 1,
        name: Some("Httpbin Get".into()),
        request: HttpRequestSpec {
            method: "GET".into(),
            url: "https://httpbin.org/anything".into(),
            query: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            timeout_secs: None,
            follow_redirects: true,
            verify_tls: true,
        },
        post_script: None,
    }
}

fn serialize_sequence(ext: &str) -> Result<String, String> {
    let doc = default_sequence_file();
    match ext {
        "json" => serde_json::to_string_pretty(&doc).map_err(|e| e.to_string()),
        "yaml" | "yml" => serde_yaml::to_string(&doc).map_err(|e| e.to_string()),
        _ => Err(format!("unsupported extension for sequence: {ext}")),
    }
}

fn serialize_request(ext: &str) -> Result<String, String> {
    let doc = default_request_file();
    match ext {
        "json" => serde_json::to_string_pretty(&doc).map_err(|e| e.to_string()),
        "yaml" | "yml" => serde_yaml::to_string(&doc).map_err(|e| e.to_string()),
        _ => Err(format!("unsupported extension for request: {ext}")),
    }
}

#[derive(Clone, Copy)]
enum TemplateKind {
    Sequence,
    Request,
}

/// `nativedoctor new --sequence PATH | --request PATH`
pub fn run_new(sequence: Option<&PathBuf>, request: Option<&PathBuf>) -> Result<(), String> {
    match (sequence, request) {
        (Some(path), None) => write_template(path, TemplateKind::Sequence),
        (None, Some(path)) => write_template(path, TemplateKind::Request),
        (None, None) => Err(
            "specify one of: --sequence <PATH> or --request <PATH> (see nativedoctor new --help)"
                .into(),
        ),
        (Some(_), Some(_)) => Err("pass only one of --sequence or --request".into()),
    }
}

fn write_template(path: &Path, kind: TemplateKind) -> Result<(), String> {
    if path.exists() {
        return Err(format!(
            "refusing to overwrite existing file: {}",
            path.display()
        ));
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    let content = match (kind, ext.as_str()) {
        (TemplateKind::Sequence, "json" | "yaml" | "yml") => serialize_sequence(ext.as_str())?,
        (TemplateKind::Request, "json" | "yaml" | "yml") => serialize_request(ext.as_str())?,
        (TemplateKind::Sequence, _) => {
            return Err("--sequence path must end with .json, .yaml, or .yml".into());
        }
        (TemplateKind::Request, _) => {
            return Err("--request path must end with .json, .yaml, or .yml".into());
        }
    };
    std::fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

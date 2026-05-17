//! Map OpenAPI paths and operations into [`nd_core::RequestFile`] values.

use std::collections::{HashMap, HashSet};

use nd_core::model::request::{
    HttpRequestSpec, RequestBody, RequestBodyKind, RequestBodyStructured, RequestFile,
};
use openapiv3::{
    OpenAPI, Operation, Parameter, PathItem, ReferenceOr, RequestBody as OasRequestBody,
};

use crate::error::{Error, Result};

/// Convert `{param}` path segments to nativedoctor `${param}` template syntax.
pub fn path_to_url_template(path: &str) -> String {
    let re = regex::Regex::new(r"\{([^}]+)\}").expect("valid regex");
    re.replace_all(path, |caps: &regex::Captures| {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("${{{name}}}")
    })
    .to_string()
}

fn base_url(api: &OpenAPI) -> String {
    api.servers
        .first()
        .map(|s| s.url.trim_end_matches('/').to_string())
        .unwrap_or_else(|| nd_constants::OPENAPI_GENERATE_BASE_URL_PLACEHOLDER.to_string())
}

fn merge_parameters(path_item: &PathItem, operation: &Operation) -> Result<Vec<Parameter>> {
    let mut out = Vec::new();
    for p in &path_item.parameters {
        match p {
            ReferenceOr::Item(param) => out.push(param.clone()),
            ReferenceOr::Reference { .. } => return Err(Error::ParameterRef),
        }
    }
    for p in &operation.parameters {
        match p {
            ReferenceOr::Item(param) => out.push(param.clone()),
            ReferenceOr::Reference { .. } => return Err(Error::ParameterRef),
        }
    }
    Ok(out)
}

fn apply_parameters(
    params: &[Parameter],
    query: &mut HashMap<String, String>,
    headers: &mut HashMap<String, String>,
) {
    for p in params {
        match p {
            Parameter::Query { parameter_data, .. } => {
                let name = parameter_data.name.clone();
                let val = if parameter_data.required {
                    format!("${{{name}}}")
                } else {
                    String::new()
                };
                query.insert(name, val);
            }
            Parameter::Header { parameter_data, .. } => {
                let name = parameter_data.name.clone();
                let lower = name.to_ascii_lowercase();
                if matches!(lower.as_str(), "accept" | "content-type" | "authorization") {
                    continue;
                }
                let val = if parameter_data.required {
                    format!("${{{name}}}")
                } else {
                    String::new()
                };
                headers.insert(name, val);
            }
            Parameter::Path { .. } | Parameter::Cookie { .. } => {}
        }
    }
}

fn json_request_body(op: &Operation) -> Result<Option<RequestBody>> {
    let Some(rb_ref) = &op.request_body else {
        return Ok(None);
    };
    let body: &OasRequestBody = match rb_ref {
        ReferenceOr::Item(b) => b,
        ReferenceOr::Reference { .. } => return Err(Error::RequestBodyRef),
    };
    let Some((_, _mt)) = body.content.iter().find(|(k, _)| {
        let k = k.as_str();
        k == "application/json" || k.starts_with("application/json;")
    }) else {
        return Ok(None);
    };
    Ok(Some(RequestBody::Structured(RequestBodyStructured {
        body_type: RequestBodyKind::Json,
        content: serde_json::json!({}),
    })))
}

/// Build a [`RequestFile`] for one operation.
pub fn operation_to_request_file(
    api: &OpenAPI,
    path_template: &str,
    method: &str,
    operation: &Operation,
    path_item: &PathItem,
) -> Result<RequestFile> {
    let base = base_url(api);
    let path_part = path_to_url_template(path_template);
    let url = if path_part.starts_with('/') {
        format!("{base}{path_part}")
    } else {
        format!("{base}/{path_part}")
    };

    let mut query = HashMap::new();
    let mut headers = HashMap::new();
    let params = merge_parameters(path_item, operation)?;
    apply_parameters(&params, &mut query, &mut headers);

    let body = json_request_body(operation)?;

    let name = operation
        .operation_id
        .clone()
        .or_else(|| Some(fallback_operation_name(method, path_template)));

    let deprecated = operation.deprecated;

    Ok(RequestFile {
        version: nd_constants::REQUEST_FILE_DEFAULT_VERSION.to_string(),
        name,
        request: HttpRequestSpec {
            method: method.to_ascii_uppercase(),
            url,
            summary: operation.summary.clone(),
            description: operation.description.clone(),
            tags: operation.tags.clone(),
            deprecated,
            query,
            headers,
            body,
            timeout_secs: None,
            follow_redirects: true,
            verify_tls: true,
        },
        ..Default::default()
    })
}

fn fallback_operation_name(method: &str, path: &str) -> String {
    let slug = path
        .trim_start_matches('/')
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let slug = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    format!("{}-{}", method.to_ascii_lowercase(), slug)
}

/// Stable filename stem (no extension): prefer `operationId`, else fallback from method + path.
pub fn file_stem(operation: &Operation, method: &str, path: &str) -> String {
    let raw = operation
        .operation_id
        .clone()
        .unwrap_or_else(|| fallback_operation_name(method, path));
    sanitize_stem(&raw)
}

fn sanitize_stem(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
            out.push(c.to_ascii_lowercase());
        } else {
            out.push('-');
        }
    }
    let out = out
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if out.is_empty() {
        "request".into()
    } else {
        out
    }
}

/// Allocate unique `stem` by appending `_2`, `_3`, … on collision.
#[cfg(test)]
mod tests {
    use super::path_to_url_template;

    #[test]
    fn path_braces_become_dollar_templates() {
        assert_eq!(
            path_to_url_template("/pets/{petId}/x/{y}"),
            "/pets/${petId}/x/${y}"
        );
    }
}

/// Returns `stem` if unused, else `stem_2`, `stem_3`, … until unique (mutates `used`).
pub fn unique_stem(stem: &str, used: &mut HashSet<String>) -> String {
    if !used.contains(stem) {
        used.insert(stem.to_string());
        return stem.to_string();
    }
    let mut n = 2;
    loop {
        let candidate = format!("{stem}_{n}");
        if !used.contains(&candidate) {
            used.insert(candidate.clone());
            return candidate;
        }
        n += 1;
    }
}

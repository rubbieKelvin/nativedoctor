//! Expand templates and merge computed headers into a [`PreparedRequest`](super::types::PreparedRequest).

use std::collections::HashMap;

use base64::Engine;
use nd_constants::{
    HTTP_HEADER_ACCEPT, HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_USER_AGENT, USER_AGENT,
};
use reqwest::Method;

use super::types::PreparedRequest;
use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::model::{
    content_type_for_body_kind, HttpRequestSpec, RequestBody, RequestBodyKind,
    RequestBodyStructured, RequestFile,
};
use crate::template::{expand_json_value, expand_string};

/// Adds default `User-Agent`, `Accept`, and (when applicable) `Content-Type` before user headers.
pub(crate) fn generate_computed_headers(spec: &HttpRequestSpec) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(HTTP_HEADER_USER_AGENT.to_string(), USER_AGENT.to_string());
    headers.insert(HTTP_HEADER_ACCEPT.to_string(), "*/*".to_string());

    let Some(body) = &spec.body else {
        return headers;
    };

    let has_content_type = spec
        .headers
        .keys()
        .any(|k| k.to_ascii_lowercase() == HTTP_HEADER_CONTENT_TYPE);

    if !has_content_type {
        let ct = match body {
            RequestBody::Json(_) => content_type_for_body_kind(RequestBodyKind::Json),
            RequestBody::Text(_) => content_type_for_body_kind(RequestBodyKind::Text),
            RequestBody::Structured(n) => content_type_for_body_kind(n.body_type),
        };
        if let Some(ct) = ct {
            headers.insert(HTTP_HEADER_CONTENT_TYPE.to_string(), ct.to_string());
        }
    }

    headers
}

fn structured_content_string(
    env: &RuntimeEnv,
    content: &serde_json::Value,
    ctx: &str,
) -> Result<String> {
    match content {
        serde_json::Value::String(s) => expand_string(env, s),
        _ => Err(Error::InvalidRequest(format!(
            "{ctx}: `content` must be a JSON string for this body type"
        ))),
    }
}

fn expand_structured_body(env: &RuntimeEnv, s: &RequestBodyStructured) -> Result<Option<Vec<u8>>> {
    match s.body_type {
        RequestBodyKind::Json | RequestBodyKind::Graphql => {
            let expanded = expand_json_value(env, &s.content)?;
            let bytes = serde_json::to_vec(&expanded).map_err(|e| {
                Error::InvalidRequest(format!("failed to serialize JSON body: {e}"))
            })?;
            Ok(Some(bytes))
        }
        RequestBodyKind::Binary => {
            let b64 = structured_content_string(env, &s.content, "binary body")?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64.trim().as_bytes())
                .map_err(|e| Error::InvalidRequest(format!("binary body: invalid base64: {e}")))?;
            Ok(Some(bytes))
        }
        RequestBodyKind::Text
        | RequestBodyKind::Xml
        | RequestBodyKind::Other
        | RequestBodyKind::XWwwFormUrlencoded
        | RequestBodyKind::FormData => {
            let raw = structured_content_string(env, &s.content, "body")?;
            let bytes = expand_string(env, &raw)?.into_bytes();
            Ok(Some(bytes))
        }
        RequestBodyKind::None => {
            unreachable!("This case should be handled when expanding request body")
        }
    }
}

fn expand_request_body(env: &RuntimeEnv, body: &RequestBody) -> Result<Option<Vec<u8>>> {
    match body {
        RequestBody::Structured(s) => expand_structured_body(env, s),
        RequestBody::Text(t) => Ok(Some(expand_string(env, t)?.into_bytes())),
        RequestBody::Json(v) => {
            let expanded = expand_json_value(env, v)?;
            let bytes = serde_json::to_vec(&expanded).map_err(|e| {
                Error::InvalidRequest(format!("failed to serialize JSON body: {e}"))
            })?;
            Ok(Some(bytes))
        }
    }
}

/// Applies env expansion to method, URL, query, headers, and body.
pub(crate) fn expand_http_request(
    env: &RuntimeEnv,
    spec: &HttpRequestSpec,
) -> Result<PreparedRequest> {
    let method = Method::from_bytes(spec.method.to_uppercase().as_bytes())
        .map_err(|_| Error::InvalidRequest(format!("unsupported HTTP method: {}", spec.method)))?;
    let url = expand_string(env, &spec.url)?;
    let mut query = Vec::new();

    for (k, v) in &spec.query {
        query.push((expand_string(env, k)?, expand_string(env, v)?));
    }

    let mut headers = Vec::new();
    let mut computed_headers = generate_computed_headers(spec);

    for (k, v) in &spec.headers {
        computed_headers.insert(k.clone().to_lowercase(), v.clone());
    }

    for (k, v) in &computed_headers {
        headers.push((expand_string(env, k)?, expand_string(env, v)?));
    }

    let body = match &spec.body {
        None => None,
        Some(b) => expand_request_body(env, b)?,
    };

    let timeout_secs = spec
        .timeout_secs
        .unwrap_or(RequestFile::default_timeout_secs());

    Ok(PreparedRequest {
        method,
        url,
        query,
        headers,
        body,
        timeout_secs,
        follow_redirects: spec.follow_redirects,
        verify_tls: spec.verify_tls,
    })
}

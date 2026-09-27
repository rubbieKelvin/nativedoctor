//! Expand templates and merge computed headers into a [`PreparedRequest`](super::types::PreparedRequest).

use std::collections::HashMap;

use base64::Engine;
use nd_constants::{
    HTTP_HEADER_ACCEPT, HTTP_HEADER_CONTENT_TYPE, HTTP_HEADER_USER_AGENT, USER_AGENT,
};
use nd_core::env::RuntimeEnv;
use nd_core::model::{ResolvedRequest, SendBody, SendBodyKind, SendBodyStructured};
use nd_core::utils::template::{expand_json_value_with_overrides, expand_string_with_overrides};
use reqwest::Method;

use super::types::PreparedRequest;
use crate::error::{Error, Result};

/// Adds default `User-Agent`, `Accept`, and (when applicable) `Content-Type` before user headers.
pub(crate) fn generate_computed_headers(spec: &ResolvedRequest) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(HTTP_HEADER_USER_AGENT.to_string(), USER_AGENT.to_string());
    headers.insert(HTTP_HEADER_ACCEPT.to_string(), "*/*".to_string());

    let has_content_type = spec
        .headers
        .keys()
        .any(|k| k.to_ascii_lowercase() == HTTP_HEADER_CONTENT_TYPE);

    if !has_content_type {
        let content_type = spec.get_contenttype_hint();
        if let Some(ct) = content_type {
            headers.insert(HTTP_HEADER_CONTENT_TYPE.to_string(), ct.to_string());
        }
    }

    return headers;
}

fn structured_content_string(
    env: &RuntimeEnv,
    overrides: Option<&HashMap<String, String>>,
    content: &serde_json::Value,
    ctx: &str,
) -> Result<String> {
    match content {
        serde_json::Value::String(s) => {
            Ok(expand_string_with_overrides(env, overrides, s).map_err(Error::Core)?)
        }
        _ => Err(Error::InvalidRequest(format!(
            "{ctx}: `content` must be a JSON string for this body type"
        ))),
    }
}

fn expand_structured_body(
    env: &RuntimeEnv,
    overrides: Option<&HashMap<String, String>>,
    s: &SendBodyStructured,
) -> Result<Option<Vec<u8>>> {
    match s.body_type {
        SendBodyKind::Json | SendBodyKind::Graphql => {
            let expanded =
                expand_json_value_with_overrides(env, overrides, &s.content).map_err(Error::Core)?;
            let bytes = serde_json::to_vec(&expanded).map_err(|e| {
                Error::InvalidRequest(format!("failed to serialize JSON body: {e}"))
            })?;
            Ok(Some(bytes))
        }
        SendBodyKind::Binary => {
            let b64 = structured_content_string(env, overrides, &s.content, "binary body")?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64.trim().as_bytes())
                .map_err(|e| Error::InvalidRequest(format!("binary body: invalid base64: {e}")))?;
            Ok(Some(bytes))
        }
        SendBodyKind::Text
        | SendBodyKind::Xml
        | SendBodyKind::Other
        | SendBodyKind::XWwwFormUrlencoded
        | SendBodyKind::FormData => {
            let raw = structured_content_string(env, overrides, &s.content, "body")?;
            let bytes = expand_string_with_overrides(env, overrides, &raw)
                .map_err(Error::Core)?
                .into_bytes();
            Ok(Some(bytes))
        }
        SendBodyKind::None => {
            unreachable!("This case should be handled when expanding request body")
        }
    }
}

fn expand_request_body(
    env: &RuntimeEnv,
    overrides: Option<&HashMap<String, String>>,
    body: &SendBody,
) -> Result<Option<Vec<u8>>> {
    match body {
        SendBody::Structured(s) => expand_structured_body(env, overrides, s),
        SendBody::Text(t) => Ok(Some(
            expand_string_with_overrides(env, overrides, t)
                .map_err(Error::Core)?
                .into_bytes(),
        )),
        SendBody::Json(v) => {
            let expanded = expand_json_value_with_overrides(env, overrides, v).map_err(Error::Core)?;
            let bytes = serde_json::to_vec(&expanded).map_err(|e| {
                Error::InvalidRequest(format!("failed to serialize JSON body: {e}"))
            })?;
            Ok(Some(bytes))
        }
    }
}

/// Applies env expansion to method, URL, query, headers, and body. Pass `overrides: None` for env-only expansion.
pub fn expand_request_with_overrides(
    env: &RuntimeEnv,
    spec: &ResolvedRequest,
    overrides: Option<&HashMap<String, String>>,
) -> Result<PreparedRequest> {
    let method = Method::from_bytes(spec.method.as_str().as_bytes())
        .expect("HttpMethod::as_str is always a valid HTTP method token");
    let url = expand_string_with_overrides(env, overrides, &spec.url).map_err(Error::Core)?;
    let mut query = Vec::new();

    for (k, v) in &spec.query {
        query.push((
            expand_string_with_overrides(env, overrides, k).map_err(Error::Core)?,
            expand_string_with_overrides(env, overrides, v).map_err(Error::Core)?,
        ));
    }

    let mut headers = Vec::new();
    let mut computed_headers = generate_computed_headers(spec);

    for (k, v) in &spec.headers {
        computed_headers.insert(k.clone().to_lowercase(), v.clone());
    }

    for (k, v) in &computed_headers {
        headers.push((
            expand_string_with_overrides(env, overrides, k).map_err(Error::Core)?,
            expand_string_with_overrides(env, overrides, v).map_err(Error::Core)?,
        ));
    }

    let body = match &spec.body {
        None => None,
        Some(b) => expand_request_body(env, overrides, b)?,
    };

    let timeout_secs = spec
        .timeout_secs
        .unwrap_or_else(ResolvedRequest::default_timeout_secs);

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

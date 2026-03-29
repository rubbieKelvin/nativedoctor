//! Build [`reqwest::Client`] and perform the HTTP round-trip.

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{redirect, Client, Url};
use tracing::debug;

use super::types::PreparedRequest;
use crate::error::{Error, Result};
use crate::model::{HttpRequestSpec, RequestFile};

/// Builds a client from timeout, redirect, and TLS verification fields on the spec.
pub(crate) fn build_client(spec: &HttpRequestSpec) -> Result<Client> {
    let timeout_secs = spec
        .timeout_secs
        .unwrap_or(RequestFile::default_timeout_secs());
    Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .redirect(if spec.follow_redirects {
            redirect::Policy::default()
        } else {
            redirect::Policy::none()
        })
        .danger_accept_invalid_certs(!spec.verify_tls)
        .build()
        .map_err(Error::Http)
}

/// Appends query pairs to `base` (must be a valid URL).
pub(crate) fn merge_url_query(base: &str, query: &[(String, String)]) -> Result<String> {
    if query.is_empty() {
        return Ok(base.to_string());
    }
    let mut url = Url::parse(base)
        .map_err(|e| Error::InvalidRequest(format!("invalid URL after expansion: {base}: {e}")))?;
    for (k, v) in query {
        url.query_pairs_mut().append_pair(k, v);
    }
    Ok(url.to_string())
}

pub(crate) fn header_map(pairs: &[(String, String)]) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    for (k, v) in pairs {
        let name = HeaderName::from_bytes(k.as_bytes())
            .map_err(|_| Error::InvalidRequest(format!("invalid header name: {k}")))?;
        let value = HeaderValue::from_str(v)
            .map_err(|_| Error::InvalidRequest(format!("invalid header value for {k}")))?;
        map.insert(name, value);
    }
    Ok(map)
}

pub(crate) async fn send_request(
    client: &Client,
    prep: &PreparedRequest,
) -> Result<reqwest::Response> {
    let full_url = merge_url_query(&prep.url, &prep.query)?;
    debug!(
        method = %prep.method,
        url = %full_url,
        body_len = prep.body.as_ref().map(|b| b.len()).unwrap_or(0),
        "sending HTTP request"
    );
    let mut req = client.request(prep.method.clone(), &full_url);
    let hdrs = header_map(&prep.headers)?;

    req = req.headers(hdrs);
    if let Some(b) = &prep.body {
        req = req.body(b.clone());
    }
    req.send().await.map_err(Error::Http)
}

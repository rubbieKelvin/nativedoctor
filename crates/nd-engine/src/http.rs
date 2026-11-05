use std::error::Error as _;
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use nd_constants::DEFAULT_TIMEOUT_SECS;
use nd_core::{PreparedBody, PreparedRequest};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::multipart;
use reqwest::redirect::Policy;
use reqwest::{Client, Method};
use tokio::sync::mpsc::Sender;

use crate::cookies::CookieJar;
use crate::event::{Error, Event};

/// Sends `request` and emits its events. Dropping the receiver cancels the request.
/// A `Cookie` header on the request takes priority over the jar.
pub async fn send(
    request: PreparedRequest,
    cookies: Option<Arc<CookieJar>>,
    events: Sender<Event>,
) -> Result<(), Error> {
    let started = Instant::now();
    let limit = time_limit(&request);
    let max_redirects = request.max_redirects;

    let builder = build_request(request, cookies)?;
    let response = with_limit(limit, builder.send())
        .await?
        .map_err(|e| classify(e, limit, max_redirects))?;

    let headers: Vec<(String, String)> = response
        .headers()
        .iter()
        .map(|(name, value)| {
            let text = String::from_utf8_lossy(value.as_bytes()).to_string();
            (name.to_string(), text)
        })
        .collect();

    // A quiet event stream is normal, so only an explicit timeout applies to it.
    let is_event_stream = headers.iter().any(|(name, value)| {
        name == CONTENT_TYPE.as_str() && value.starts_with("text/event-stream")
    });
    let body_limit = if is_event_stream && limit == Some(default_timeout()) {
        None
    } else {
        limit
    };

    let head = Event::Head {
        status: response.status().as_u16(),
        reason: response.status().canonical_reason().map(str::to_string),
        version: format!("{:?}", response.version()),
        headers,
    };
    if events.send(head).await.is_err() {
        return Ok(());
    }

    let mut stream = response.bytes_stream();
    while let Some(chunk) = with_limit(body_limit, stream.next()).await? {
        let chunk = chunk.map_err(|e| classify(e, limit, max_redirects))?;
        if events.send(Event::Chunk(chunk)).await.is_err() {
            return Ok(());
        }
    }

    let elapsed = started.elapsed();
    let _ = events.send(Event::Finished { elapsed }).await;
    return Ok(());
}

fn default_timeout() -> Duration {
    Duration::from_secs(DEFAULT_TIMEOUT_SECS)
}

fn time_limit(request: &PreparedRequest) -> Option<Duration> {
    return match request.timeout {
        Some(Duration::ZERO) => None,
        Some(timeout) => Some(timeout),
        None => Some(default_timeout()),
    };
}

/// Applies `limit` to one step: getting the response head, or waiting for the next chunk.
async fn with_limit<F: Future>(limit: Option<Duration>, step: F) -> Result<F::Output, Error> {
    let Some(limit) = limit else {
        return Ok(step.await);
    };
    return tokio::time::timeout(limit, step)
        .await
        .map_err(|_| Error::Timeout(limit));
}

fn build_request(
    request: PreparedRequest,
    cookies: Option<Arc<CookieJar>>,
) -> Result<reqwest::RequestBuilder, Error> {
    let policy = if request.follow_redirects {
        Policy::limited(request.max_redirects)
    } else {
        Policy::none()
    };
    let mut client = Client::builder()
        .redirect(policy)
        .tls_danger_accept_invalid_certs(!request.verify_tls);
    if let Some(jar) = cookies {
        client = client.cookie_provider(jar);
    }
    let client = client.build().map_err(|e| Error::Other(e.to_string()))?;

    let method = Method::from_bytes(request.method.as_bytes())
        .map_err(|_| Error::Invalid(format!("bad method `{}`", request.method)))?;

    let mut headers = HeaderMap::new();
    for (name, value) in &request.headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| Error::Invalid(format!("bad header name `{name}`")))?;
        let header_value = HeaderValue::from_str(value)
            .map_err(|_| Error::Invalid(format!("bad value for header `{name}`")))?;
        headers.append(header_name, header_value);
    }

    let builder = client.request(method, request.url).headers(headers);
    return match request.body {
        PreparedBody::None => Ok(builder),
        PreparedBody::Bytes(bytes) => Ok(builder.body(bytes)),
        PreparedBody::Multipart(parts) => {
            let mut form = multipart::Form::new();
            for part in parts {
                let mut piece = multipart::Part::bytes(part.data);
                if let Some(filename) = part.filename {
                    piece = piece.file_name(filename);
                }
                if let Some(content_type) = part.content_type {
                    piece = piece.mime_str(&content_type).map_err(|_| {
                        Error::Invalid(format!("bad content type `{content_type}`"))
                    })?;
                }
                form = form.part(part.name, piece);
            }
            Ok(builder.multipart(form))
        }
    };
}

fn classify(error: reqwest::Error, limit: Option<Duration>, max_redirects: usize) -> Error {
    if error.is_timeout() {
        return Error::Timeout(limit.unwrap_or_default());
    }
    if error.is_redirect() {
        return Error::TooManyRedirects(max_redirects);
    }
    let is_connect = error.is_connect();
    // Without the URL, since it can contain secrets.
    let error = error.without_url();
    let message = match error.source() {
        Some(source) => format!("{error}: {source}"),
        None => error.to_string(),
    };
    if is_connect {
        return Error::Connect(message);
    }
    return Error::Other(message);
}

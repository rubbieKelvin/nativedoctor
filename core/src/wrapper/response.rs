use bytes::Bytes;
use reqwest::header::HeaderMap;
use reqwest::{Error, Response, StatusCode, Url, Version};
use std::net::SocketAddr; // For efficient, clonable byte storage

/// A clonable wrapper for `reqwest::Response`.
///
/// This struct consumes a `reqwest::Response` and stores its relevant data
/// (URL, status, version, headers, remote address, and the full body) in memory.
/// This allows the response data to be cloned, which is not possible with
/// the original `reqwest::Response` due to its streaming nature.
#[derive(Debug, Clone)]
pub struct ClonableResponse {
    pub url: Url,
    pub status: StatusCode,
    pub version: Version,
    pub headers: HeaderMap,
    pub remote_addr: Option<SocketAddr>,
    pub body: Bytes, // Stores the response body as bytes
}

impl ClonableResponse {
    /// Creates a new `ClonableResponse` from a `reqwest::Response`.
    ///
    /// This function is asynchronous because it needs to read and buffer
    /// the entire response body. The original `reqwest::Response` is consumed.
    ///
    /// # Arguments
    ///
    /// * `response`: The `reqwest::Response` to consume and buffer.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ClonableResponse` if successful,
    /// or a `reqwest::Error` if reading the response fails.
    pub async fn from_reqwest_response(response: Response) -> Result<Self, Error> {
        // Extract metadata before consuming the body
        let url = response.url().clone();
        let status = response.status();
        let version = response.version();
        let headers = response.headers().clone();
        let remote_addr = response.remote_addr();

        // Consume the response body and store it.
        // This must be done after accessing other parts of the response that
        // don't consume it, or if methods like .json() or .text() were used prior,
        // ensure the body wasn't already consumed.
        let body_bytes = response.bytes().await?;

        Ok(Self {
            url,
            status,
            version,
            headers,
            remote_addr,
            body: body_bytes,
        })
    }

    /// Returns the response body as a `String`, attempting UTF-8 conversion.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `String` if UTF-8 conversion is successful,
    /// or a `FromUtf8Error` otherwise.
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.to_vec())
    }

    /// Returns a slice of the response body bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }
}

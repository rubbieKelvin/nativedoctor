use reqwest::{header::HeaderMap, Error as ReqwestError, Method, Response, StatusCode, Url};
use std::fmt;

/// Holds data extracted from an HTTP response.
///
/// The response body is populated asynchronously after the wrapper is created.
/// The method of the original request needs to be set separately if required.
pub struct ResponseWrapper {
    /// The final URL of the response, after any redirects.
    pub url: Url,
    /// The HTTP status code of the response.
    pub status: StatusCode,
    /// The headers from the response.
    pub headers: HeaderMap,

    /// The HTTP method of the original request that led to this response.
    /// This needs to be set manually as it's not directly available from `reqwest::Response`.
    pub request_method: Option<Method>,

    // Internally holds the response object to extract its body asynchronously.
    // This field is consumed when `populate_body` is called.
    // It's private as its lifecycle is managed internally.
    _pending_response: Option<Response>,

    /// The body of the response, as a String.
    /// This will be `None` until `populate_body` is called and successfully completes.
    pub body: Option<String>,
}

/// Implements the conversion from a `reqwest::Response` to a `ResponseWrapper`.
///
/// This method is synchronous and captures immediately available data like URL,
/// status, and headers. The response body is not read here; instead, the
/// `Response` object is stored internally for later asynchronous processing
/// by the `populate_body` method. The `request_method` will be `None`.
impl From<Response> for ResponseWrapper {
    fn from(response: Response) -> Self {
        ResponseWrapper {
            url: response.url().clone(),
            status: response.status(),
            headers: response.headers().clone(),
            request_method: None, // Not available directly from reqwest::Response
            _pending_response: Some(response), // Store the response for async body extraction
            body: None,           // Body is not yet populated
        }
    }
}

/// Error types that can occur when populating the response body.
#[derive(Debug)]
pub enum PopulateBodyError {
    /// Indicates that `populate_body` was called when the body was already populated.
    AlreadyPopulated,
    /// Indicates that `populate_body` was called when no pending response was available
    /// (e.g., called twice, or after a previous failed attempt that consumed the response).
    NoPendingResponse,
    /// An error occurred while trying to read the response body using reqwest.
    Reqwest(ReqwestError),
}

impl fmt::Display for PopulateBodyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PopulateBodyError::AlreadyPopulated => {
                write!(f, "Response body has already been populated.")
            }
            PopulateBodyError::NoPendingResponse => {
                write!(f, "No pending response available to populate body.")
            }
            PopulateBodyError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
        }
    }
}

impl std::error::Error for PopulateBodyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PopulateBodyError::Reqwest(err) => Some(err),
            _ => None,
        }
    }
}

// Allow converting ReqwestError into PopulateBodyError for convenience.
impl From<ReqwestError> for PopulateBodyError {
    fn from(err: ReqwestError) -> Self {
        PopulateBodyError::Reqwest(err)
    }
}

impl ResponseWrapper {
    /// Sets the HTTP method of the original request.
    /// This is useful because `reqwest::Response` doesn't carry this information.
    ///
    /// # Arguments
    /// * `method` - The `reqwest::Method` of the original request.
    pub fn set_request_method(&mut self, method: Method) {
        self.request_method = Some(method);
    }

    /// Asynchronously extracts the body from the stored `reqwest::Response`,
    /// populates the `body` field of this wrapper, and consumes the internal response.
    ///
    /// This method should ideally be called once.
    ///
    /// # Returns
    /// * `Ok(())` - If the body was successfully populated.
    /// * `Err(PopulateBodyError::AlreadyPopulated)` - If the body was already populated.
    /// * `Err(PopulateBodyError::NoPendingResponse)` - If there was no internal response object to process.
    /// * `Err(PopulateBodyError::Reqwest(e))` - If reading the response body failed.
    pub async fn populate_body(&mut self) -> Result<(), PopulateBodyError> {
        if self.body.is_some() {
            return Err(PopulateBodyError::AlreadyPopulated);
        }

        if let Some(response) = self._pending_response.take() {
            // Asynchronously read the response body as text.
            match response.text().await {
                Ok(text_body) => {
                    self.body = Some(text_body);
                    Ok(())
                }
                Err(e) => Err(PopulateBodyError::Reqwest(e)),
            }
        } else {
            // This means _pending_response was None, which shouldn't happen if body is None
            // unless populate_body was called after a previous failure that consumed _pending_response,
            // or if the struct was manually created without a _pending_response.
            Err(PopulateBodyError::NoPendingResponse)
        }
    }

    /// Checks if the body has been populated.
    pub fn is_body_populated(&self) -> bool {
        self.body.is_some()
    }

    /// An alternative asynchronous constructor that fully populates the `ResponseWrapper`,
    /// including the body and optionally the request method.
    ///
    /// This is often more convenient if you're not strictly bound to the `From<Response>` trait
    /// for instantiation and can perform async operations at the point of creation.
    ///
    /// # Arguments
    /// * `response` - The `reqwest::Response` object.
    /// * `request_method` - An `Option<reqwest::Method>` for the original request.
    ///
    /// # Returns
    /// A `Result` containing the fully populated `ResponseWrapper` or a `ReqwestError`.
    pub async fn new_fully_populated(
        response: Response,
        request_method: Option<Method>,
    ) -> Result<Self, ReqwestError> {
        let url = response.url().clone();
        let status = response.status();
        let headers = response.headers().clone();

        // Asynchronously read the body here.
        let body_text = response.text().await?;

        Ok(Self {
            url,
            status,
            headers,
            request_method,
            _pending_response: None, // The response is consumed for its body.
            body: Some(body_text),
        })
    }
}

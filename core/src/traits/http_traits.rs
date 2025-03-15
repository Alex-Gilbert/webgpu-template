use std::{future::Future, pin::Pin};

/// An error that can occur when making an HTTP request.
/// This is a simplified version of the reqwest::Error type.
/// TODO: Add more error types as needed.
#[derive(Debug)]
pub enum HttpError {
    Other(String),
}

/// Our unified trait for making HTTP requests.
pub trait HttpRequester: Send + Sync {
    /// Makes an HTTP GET request to the given URL.
    /// The returned Future resolves to a binary buffer containing the response body.
    fn make_web_request(
        &self,
        url: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, HttpError>> + Send>>;
}

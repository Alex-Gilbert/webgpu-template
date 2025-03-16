use demo_core::traits::http_traits::{HttpError, HttpRequester};
use std::{future::Future, pin::Pin};

pub struct NativeHttpRequester;

impl HttpRequester for NativeHttpRequester {
    fn make_web_request(
        &self,
        url: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<u8>, HttpError>> + Send>> {
        let url = url.to_string();
        Box::pin(async move {
            let response = reqwest::get(&url)
                .await
                .map_err(|e| HttpError::Other(e.to_string()))?;
            let bytes = response
                .bytes()
                .await
                .map_err(|e| HttpError::Other(e.to_string()))?;
            Ok(bytes.to_vec())
        })
    }
}

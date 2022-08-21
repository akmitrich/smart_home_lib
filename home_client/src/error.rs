use stp::client::RequestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HomeError {
    #[error("Request error: {0}.")]
    WhenRequested(#[from] RequestError),
    #[error("Error in response: {0}.")]
    ResponseErr(String),
    #[error("Bad response.")]
    BadResponse,
}

pub type HomeResult<T> = Result<T, HomeError>;

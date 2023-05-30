/// Enumerates errors that could occur while notifying on a Mattermost instance.
#[derive(thiserror::Error, Debug)]
pub enum MattermostError {
    /// Chained I/O errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Chained Reqwest errors
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// Chained Reqwest errors
    #[error(transparent)]
    HeaderError(#[from] reqwest::header::ToStrError),

    /// Chained Reqwest errors
    #[error(transparent)]
    HeaderValueError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("No token received from login credentials")]
    ErrorToken(),

    // No match found when capturing the number with the regex
    #[error("Error with request status : {0}")]
    RequestError(String),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Definition of the commit analyzer result
pub type MattermostResult<R> = Result<R, MattermostError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

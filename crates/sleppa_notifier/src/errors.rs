/// Enumerates errors that could occur while notifying a new published release.
#[derive(thiserror::Error, Debug)]
pub enum NotifierError {
    /// No match found when capturing the number with the regex
    #[error("An error occured: {0}.")]
    SendingError(String),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Definition of the commit analyzer result
pub type NotifierResult<R> = Result<R, NotifierError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

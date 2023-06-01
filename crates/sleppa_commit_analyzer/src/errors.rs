/// Error enumeration for the commit message analyzer parser module.
///
/// This list is a central structure aiming to define errors that can occur
/// while getting and parsing commit's message.
#[derive(thiserror::Error, Debug)]
pub enum CommitAnalyzerError {
    /// No release action type match found
    #[error("No release action found")]
    ErrorNoMatching(),

    /// Commit's message is not correct
    #[error("No release action found")]
    InvalidMessage(),

    /// Missing key/value pair in the Context
    #[error("Missing key in context : {0}")]
    InvalidContext(String),

    /// Chained sleppa commit analyzer configuration errors
    #[error(transparent)]
    ConfigurationError(#[from] crate::configuration::errors::ConfigurationError),
}

/// Definition of the commit analyzer result
pub type CommitAnalyzerResult<R> = Result<R, CommitAnalyzerError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

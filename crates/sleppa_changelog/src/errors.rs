/// Enumerates errors that could occur when generating changelog file.
///
/// This list is a central structure aiming to define errors that can occur
/// while generating the changelog.
#[derive(thiserror::Error, Debug)]
pub enum ChangelogError {
    /// Chained I/O errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Chained time format description error
    #[error(transparent)]
    InvalidFormatDescription(#[from] time::error::InvalidFormatDescription),

    /// Chained time format error
    #[error(transparent)]
    InvalidFormat(#[from] time::error::Format),

    // None found in context
    #[error("None found in context : {0}")]
    ErrorContextNone(String),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Result type alias returned by function.
pub type ChangelogResult<R> = Result<R, ChangelogError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

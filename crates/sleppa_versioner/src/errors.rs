/// Enumerates errors that could occur while incrementing a version.
#[derive(thiserror::Error, Debug)]
pub enum VersionerError {
    /// Chained I/O errors
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Chained regex error
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    // Chained errors occurring when parsing an integer
    #[error(transparent)]
    ParsingError(#[from] std::num::ParseIntError),

    // No match found when capturing the number with the regex
    #[error("No match found for {0}.")]
    ErrorNoMatch(String),
}

/// Definition of the commit analyzer result
pub type VersionerResult<R> = Result<R, VersionerError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

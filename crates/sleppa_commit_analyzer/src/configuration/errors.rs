/// Enumerates all errors that can occur when processing a commit analyzer configuration.
///
/// This list is a central structure aiming to define errors that can occur
/// while reading and parsing the configuration file.
#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    /// Chained I/O errors
    #[error(transparent)]
    InputOutputError(#[from] std::io::Error),

    /// Chained Toml file processing errors
    #[error(transparent)]
    ErrorReadingToml(#[from] toml::de::Error),

    /// Chained errors occurring when processing regular expressions
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    /// Chained errors occurring when processing with repositories
    #[error(transparent)]
    RepoError(#[from] sleppa_primitives::repositories::errors::RepositoryError),

    /// Wrong or no ReleaseAction found
    #[error("The release action is 'major', 'minor' or 'patch'. Found : {0}")]
    IncorrectReleaseAction(String),

    /// No match found when analyzing commit message with the grammar
    #[error("No match found.")]
    ErrorNoMatch(),

    /// Message is not correct
    #[error("Missing key in context")]
    InvalidContext(),
}

/// Definition of the configuration parser result
pub type ConfigurationResult<T> = std::result::Result<T, ConfigurationError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

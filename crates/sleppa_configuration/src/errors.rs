/// Enumerates all errors that can occur when processing with configuration.
///
/// This list is a central structure aiming to define errors that can occur
/// while reading and parsing the configuration file.
#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    /// No match found when analyzing commit message with the grammar
    #[error("No match found.")]
    ErrorNoMatch(),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Definition of the configuration parser result
pub type ConfigurationResult<T> = std::result::Result<T, ConfigurationError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

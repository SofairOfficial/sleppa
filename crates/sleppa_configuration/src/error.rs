/// Error enumeration for the configuration parser module.
///
/// This list is a central structure aiming to define errors that can occur
/// while reading and parsing the configuration file.
#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    // Chained I/O errors
    #[error(transparent)]
    InputOutputError(#[from] std::io::Error),

    // Error while reading the configuration toml file
    #[error(transparent)]
    ErrorReadingToml(#[from] toml::de::Error),

    // Error in the regex component of the library
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    #[error("The release action is 'major', 'minor' or 'patch'. Found : {0}")]
    IncorrectReleaseAction(String),
}

/// Definition of the configuration parser result
pub type ConfigurationResult<T> = std::result::Result<T, ConfigurationError>;

/// Enumerates errors that could occur when working with repositories.
///
/// This list is a central structure aiming to define errors that can occur
/// while processing with repositories.
#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    // Chained errors occuring when processing with GitHub
    #[error(transparent)]
    GithubError(#[from] octocrab::GitHubError),

    // Chained errors occuring when processing with octocrab's API
    #[error(transparent)]
    ApiError(#[from] octocrab::Error),

    // Chained errors occurring when processing regular expressions
    #[error(transparent)]
    RegexError(#[from] regex::Error),

    // Chained errors occurring when parsing an integer
    #[error(transparent)]
    ParsingError(#[from] std::num::ParseIntError),

    // Message is not correct
    #[error("Pull request name is incorrect : {0}")]
    InvalidMessage(String),
}

/// Definition of the commit analyzer result
pub type RepositoryResult<R> = Result<R, RepositoryError>;

#[cfg(test)]
/// Result type alias returned by function in unit tests.
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

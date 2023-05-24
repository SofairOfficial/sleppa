/// Enumerates errors that could occur while publishing a release.
///
/// This list is a central structure aiming to define errors that can occur
/// while publishing a release.
#[derive(thiserror::Error, Debug)]
pub enum CodeArchiverError {
    // Chained errors occuring when processing with GitHub
    #[error(transparent)]
    GithubErr(#[from] octocrab::GitHubError),

    // Chained errors occuring when processing with octocrab's API
    #[error(transparent)]
    ApiError(#[from] octocrab::Error),

    // Chained errors occurring when accessing environment variables
    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Definition of the commit analyzer result
pub type CodeArchiverResult<R> = Result<R, CodeArchiverError>;

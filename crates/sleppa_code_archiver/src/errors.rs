/// Enumerates errors that could occur while publishing a release.
///
/// This list is a central structure aiming to define errors that can occur
/// while publishing a release.
#[derive(thiserror::Error, Debug)]
pub enum CodeArchiverError {
    // Chained errors occuring when processing with octocrab's API
    #[error(transparent)]
    RepoError(#[from] sleppa_primitives::repositories::errors::RepositoryError),

    /// Missing key or value in context
    #[error("Missing key in context: {0}")]
    InvalidContext(String),
}

/// Definition of the commit analyzer result
pub type CodeArchiverResult<R> = Result<R, CodeArchiverError>;

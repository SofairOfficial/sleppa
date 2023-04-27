use octocrab::GitHubError;
use serde::Deserialize;

/// Error enumeration for the commit message analyzer parser module.
///
/// This list is a central structure aiming to define errors that can occur
/// while getting and parsing commit's message.
#[derive(thiserror::Error, Debug, Deserialize)]
pub enum CommitAnalyzerError {
    // No release action type match found
    #[error("No release action found")]
    ErrorNoMatching(),

    // Message is not correct
    #[error("No release action found")]
    InvalidMessage(),

    // Chained errors occuring when processing with GitHub
    #[error(transparent)]
    GithubError(#[from] GitHubError),
}

/// Definition of the commit analyzer result
pub type CommitAnalyzerResult<R> = Result<R, CommitAnalyzerError>;

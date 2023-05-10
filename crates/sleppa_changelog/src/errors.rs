/// Enumerates errors that could occur when generating changelog.
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
}

/// Definition of the changelog result
pub type ChangelogResult<R> = Result<R, ChangelogError>;

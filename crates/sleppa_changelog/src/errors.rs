/// Enumerates errors that could occur when working with changelog.
///
/// This list is a central structure aiming to define errors that can occur
/// while processing with changelog.
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

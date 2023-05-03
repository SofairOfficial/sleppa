//! This module defines the interface between the `sleppa_commit_analyzer` crate and the distributed version control system,
//! namely git.
//!
//! To be agnostic to the git system, the trait `Repository` defines the shared behavior to implement to use
//! sleppa_commit_analyzer.
//!
//! Moreover, a shared structure [RepositoryTag] defines the tag of a git repository system with its two basic property,
//! namely the identifier (e.g. `v3.2.1`) and the associated hash.
//!
//! In order to operate, `sleppa_commit_analyzer` needs the last tag of a repository and the commit messages since this last
//! tag.

pub mod errors;
pub mod github;

use async_trait::async_trait;
use errors::RepositoryResult;

/// Definition of a repository's tag.
pub struct RepositoryTag {
    /// Value of the tag e.g. `v3.2.1`.
    pub identifier: String,
    /// Hash of the tag.
    pub hash: String,
}

/// Trait to interface the git system used with `sleppa_commit_analyzer`.
///
/// `sleppa_commit_analyzer` needs the inner commit messages and the last tag to retrieve the
/// [ReleaseAction].
#[async_trait]
pub trait Repository {
    /// Get the repository's last tag and its sha.
    ///
    /// The output is used later to process the new tag.
    async fn get_last_tag(&self) -> RepositoryResult<RepositoryTag>;

    /// Get inner commit messages since the last tag.
    ///
    /// The output is analyzed by the commit analyzer to define the release action type.
    async fn get_inner_commits(&self) -> RepositoryResult<Vec<String>>;
}

pub mod errors;
pub mod github;

use async_trait::async_trait;
use errors::RepositoriesResult;

/// Definition of a repository's tag.
pub struct RepoTag {
    /// Value of the tag e.g. `v3.2.1`.
    pub tag_id: String,
    /// Hash of the tag.
    pub tag_hash: String,
}

#[async_trait]
pub trait Repository {
    /// Get the repository's last tag and its sha.
    ///
    /// The output is used later to process the new tag.
    async fn get_last_tag(&self) -> RepositoriesResult<RepoTag>;

    /// Get inner commit's messages since the last tag.
    ///
    /// The output is analyzed by the commit analyzer to define the release action type.
    async fn get_inner_commits_messages(&self) -> RepositoriesResult<Vec<String>>;
}

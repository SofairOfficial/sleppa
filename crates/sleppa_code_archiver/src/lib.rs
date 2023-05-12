//! Sleppa code archiver package
//!
//! This crate produces a release into a GitHub repository while publishing
//! code archives in a `.zip` and a `.tar.gz` format.
//!
//! The release needs a logged user and a tag to be published.

mod errors;

use errors::CodeArchiverResult;
use sleppa_primitives::repositories::{github::GithubRepository, *};

/// Defines the code archiver plugin and its fields
///
/// The [CodeArchiverPlugin] is composed of a [RepositoryTag] used to publish the release's tag
/// and a [GithubRepository] since it works for GitHub only.
pub struct CodeArchiverPlugin {
    /// The tag associated with the release in the GitHub repository
    pub release_tag: RepositoryTag,
    /// The repository where the release is to be published
    pub repository: GithubRepository,
}

impl CodeArchiverPlugin {
    /// Publishes a release into the GitHub repository
    ///
    /// The release is published for a given [RepositoryTag] into a [GithubRepository].
    /// The credentials are mandatory to publish a release.
    pub async fn run(&self) -> CodeArchiverResult<()> {
        // Constructs the credentials
        let token = std::env::var("GITHUB_TOKEN")?;

        // Build an octocrab instance with the provided credentials.
        let octocrab = octocrab::Octocrab::builder()
            .personal_token(token)
            .build()?;

        // Publishes the release for the given tag.
        octocrab
            .repos(&self.repository.owner, &self.repository.repo)
            .releases()
            .create(&self.release_tag.identifier)
            .target_commitish("main")
            .send()
            .await?;
        Ok(())
    }
}

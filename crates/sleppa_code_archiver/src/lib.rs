//! Sleppa code archiver package
//!
//! This crate produces a release into a git repository while publishing
//! code archives in diverse compressed format.
//!
//! The plugin needs a [CONTEXT_NEW_TAG] to load from the [Context].

mod errors;

use errors::{CodeArchiverError, CodeArchiverResult};
use sleppa_primitives::{
    repositories::{GitRepository, RepositoryTag},
    Context,
};

/// Definition of the code archiver plugin and its fields
///
/// The [CodeArchiverPlugin] is composed of a [RepositoryTag] used to publish the release's tag.
pub struct CodeArchiverPlugin {
    /// The tag associated with the release in the repository
    pub release_tag: RepositoryTag,
}

impl CodeArchiverPlugin {
    /// Publishes a release into the GitHub repository
    ///
    /// The release is published for a given [RepositoryTag].
    pub async fn run<R: GitRepository>(&self, context: &Context<R>) -> CodeArchiverResult<()> {
        let tag = match context.load_new_tag() {
            Some(value) => value,
            None => return Err(CodeArchiverError::InvalidContext("missing last tag".to_string())),
        };

        context.repository.push_release(tag).await?;

        Ok(())
    }
}

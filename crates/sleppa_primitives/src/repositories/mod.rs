//! Sleppa repositories module
//!
//! This module aims to define the behavior of a repository while working with Sleppa
//! To be agnostic to the git system, it implements the trait [Repository].
//!
//! Also, a common [RepositoryTag] structure defines the tag of a git repository system with
//! its two basic properties, namely, its identifier (e.g. `v3.2.1`) and its associated hash.
//!
//! It natively implements a [github::GithubRepository] link to work with GitHub.

pub mod errors;
pub mod github;

use crate::Commit;
use async_trait::async_trait;
use errors::RepositoryResult;

/// Definition of a repository's tag.
pub struct RepositoryTag {
    /// Value of the tag e.g. `v3.2.1` where `v{major}.{minor}.{patch}`
    pub identifier: String,
    /// long tag identifier (i.e. 40 digits long SHA-1 hash)
    pub hash: String,
}

/// Definition of a repository's user
pub struct RepositoryUser {
    pub name: String,
    pub email: String,
    // The credentials for the user. This could be a `GITHUB_TOKEN`.
    pub signing_key: String,
}

/// Trait to interface the git system used.
#[async_trait]
pub trait Repository {
    /// Get the repository's last tag and its sha.
    async fn get_last_tag(&self) -> RepositoryResult<RepositoryTag>;

    /// Get inner commit messages since the last tag.
    async fn get_inner_commits(&self) -> RepositoryResult<Vec<Commit>>;
}

impl RepositoryUser {
    /// Provides a method to create a now user from name, email and credential datas.
    pub fn new(username: String, useremail: String, credential: String) -> Self {
        RepositoryUser {
            name: username,
            email: useremail,
            signing_key: credential,
        }
    }
}

#[cfg(test)]
mod tests;

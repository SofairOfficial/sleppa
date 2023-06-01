//! Sleppa primitives package
//!
//! This crate provides convenient primitives, like, structures, modules or
//! reusable code.
//!
//! Shared datas are retrieved from a [Context] structure.
//! This [Context] should contain a [CONTEXT_COMMITS] to access the list of commits, [CONTEXT_USER]
//! to access the user, [CONTEXT_REPO] to access the repository URL, [CONTEXT_LAST_TAG] to access the last
//! tag of a repository and [CONTEXT_NEW_TAG] to access the new tag of a repository.
//!
//! [CONTEXT_COMMITS] is used by `sleppa_commit_analyzer` and `sleppa_changelog`.
//! [CONTEXT_USER] is used by `sleppa_changelog` and `sleppa_versioner`.
//! [CONTEXT_LAST_TAG] is used by `sleppa_changelog` and `sleppa_versioner`.
//! [CONTEXT_NEW_TAG] is used by `sleppa_changelog` and `sleppa_code_archiver`.
//! [CONTEXT_RELEASE_ACTION] is used by `sleppa_versioner`.
//!
//! These datas are retrieved thanks to the associated [Context]'s method.

pub mod constants;
pub mod repositories;

use constants::{CONTEXT_COMMITS, CONTEXT_LAST_TAG, CONTEXT_NEW_TAG, CONTEXT_RELEASE_ACTION, CONTEXT_USER};
use repositories::{GitRepository, RepositoryTag, RepositoryUser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The git's commit representation with its hash, message and associated [ReleaseAction]
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    /// long commit identifier (i.e. 40 digits long SHA-1 hash)
    pub hash: String,
    /// The commit message like: `feat(github): a new feature`
    pub message: String,
    /// Commit associated ReleaseAction
    pub release_action: Option<ReleaseAction>,
}

/// The context structure used to share datas between crates.
///
/// The used repository should implements the [GitRepository] trait as Sleppa works only with git.
pub struct Context<R>
where
    R: GitRepository,
{
    pub map: HashMap<String, Value>,
    pub repository: R,
}

/// Enumeration of possible values used by crates.
#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Commits(Vec<Commit>),
    User(RepositoryUser),
    Tag(RepositoryTag),
    ReleaseAction(ReleaseAction),
}

/// Enumeration of all release actions type.
#[derive(PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Clone, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseAction {
    /// Major release implying the left digit of a semantic version to be incremented (e.g. from `1.0.1` -> `2.0.0`)
    Major,
    /// Minor release implying the middle digit of a semantic version to be incremented (e.g. from `1.0.1` -> `1.1.0`)
    Minor,
    /// Patch release implying the right digit of a semantic version to be incremented (e.g. from `1.0.1` -> `1.0.2`)
    Patch,
}

impl Commit {
    /// Creates a new Commit with its hash and its message
    ///
    /// The release_action is set as None at first. It will be determined later.
    pub fn new(commitmessage: String, sha: String) -> Self {
        Commit {
            hash: sha,
            message: commitmessage,
            release_action: None,
        }
    }
}

impl<R: GitRepository> Context<R> {
    /// Loads an optionnal new [RepositoryTag] of a repository from the context
    pub fn load_new_tag(&self) -> Option<RepositoryTag> {
        self.map[CONTEXT_NEW_TAG].as_tag()
    }

    /// Loads an optionnal Vec<[Commit]> from the context
    pub fn load_commits(&self) -> Option<Vec<Commit>> {
        self.map[CONTEXT_COMMITS].as_commits()
    }

    /// Loads an optionnal [RepositoryUser] from the context
    pub fn load_user(&self) -> Option<RepositoryUser> {
        self.map[CONTEXT_USER].as_user()
    }

    /// Loads an optionnal last [RepositoryTag] of a repository from the context
    pub fn load_last_tag(&self) -> Option<RepositoryTag> {
        self.map[CONTEXT_LAST_TAG].as_tag()
    }

    /// Loads an optionnal [ReleaseAction] from the context
    pub fn load_release_action(&self) -> Option<ReleaseAction> {
        self.map[CONTEXT_RELEASE_ACTION].as_release_action()
    }
}

impl Value {
    /// Extracts the string slice value if it is a string slice.
    pub fn as_string(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(&**s),
            _ => None,
        }
    }

    /// Extracts the vector of [Commit]s from the [Value].
    pub fn as_commits(&self) -> Option<Vec<Commit>> {
        match self {
            Value::Commits(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    /// Extracts the [RepositoryTag] from the [Value].
    pub fn as_tag(&self) -> Option<RepositoryTag> {
        match self {
            Value::Tag(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Extracts the [RepositoryUser] from the [Value].
    pub fn as_user(&self) -> Option<RepositoryUser> {
        match self {
            Value::User(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// Extracts the [ReleaseAction] from the [Value].
    pub fn as_release_action(&self) -> Option<ReleaseAction> {
        match self {
            Value::ReleaseAction(s) => Some(s.clone()),
            _ => None,
        }
    }
}

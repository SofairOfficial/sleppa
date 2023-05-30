//! Sleppa primitives package
//!
//! This crate provides convenient primitives, like, structures, modules or
//! reusable code.

pub mod repositories;

use repositories::{github::GithubRepository, RepositoryTag, RepositoryUser};
use serde::{Deserialize, Serialize};

/// Defines Commit and its fields
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    /// long commit identifier (i.e. 40 digits long SHA-1 hash)
    pub hash: String,
    /// The commit message like: `feat(github): a new feature`
    pub message: String,
    /// Commit associated ReleaseAction
    pub release_action: Option<ReleaseAction>,
}

/// Defines possible values used by plugin
#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Commits(Vec<Commit>),
    User(RepositoryUser),
    Repository(GithubRepository),
    Tag(RepositoryTag),
}

/// Enumerates available release actions.
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

impl Value {
    /// Extracts the string slice value if it is a string slice.
    pub fn as_string(&self) -> Option<&str> {
        match *self {
            Value::String(ref s) => Some(&**s),
            _ => None,
        }
    }

    pub fn as_commits(&self) -> Option<Vec<Commit>> {
        match self {
            Value::Commits(s) => Some(s.to_vec()),
            _ => None,
        }
    }

    pub fn as_tag(&self) -> Option<RepositoryTag> {
        match self {
            Value::Tag(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_user(&self) -> Option<RepositoryUser> {
        match self {
            Value::User(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn as_repository(&self) -> Option<GithubRepository> {
        match self {
            Value::Repository(s) => Some(s.clone()),
            _ => None,
        }
    }
}

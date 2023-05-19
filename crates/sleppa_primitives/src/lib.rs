//! Sleppa primitives package
//!
//! This crate provides convenient primitives, like, structures, modules or
//! reusable code.

pub mod repositories;

use serde::{Deserialize, Serialize};
use sleppa_configuration::Configuration;

/// The context structure is used to pass the data from plugin to plugin.
#[derive(Default)]
pub struct Context {
    pub configuration: Configuration,
}

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

/// If the plugin needs a configuration to work, this traits defines the behavior to load this
/// configuration.
pub trait Configurable<T, V> {
    fn load(&self, input: T) -> V;
}

impl Commit {
    /// Creates a new Commit with its hash and its message
    ///
    /// The others values of [Commit] are set as None at first.
    pub fn new(commitmessage: String, sha: String) -> Self {
        Commit {
            hash: sha,
            message: commitmessage,
            release_action: None,
        }
    }
}

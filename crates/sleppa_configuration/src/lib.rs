//! Sleppa configuration management package
//!
//! This crates defines the structure useb by the crate to store datas and pass them through the process.
//! A top structure named [Context] contains a `configurations` field used to store configuration of each crate
//! if needed.
//! A [Configuration] is a second structure used to store value used by crates. It contains a field `map` which
//! is defined by a [Hashmap].
//!
//! To access a specific crate's [Configuration], a key is defined in a `constants` module in the crate.
//! Then datas are retrieved with a key also defined in the constants module.
//!
//! For example, the `sleppa_configuration` crate has some value shared by multltiple crate.
//! Therefore, it needs a [Configuration] in the [Context].
//! To do so, the [CONFIGURATION_KEY] allows to access the [Configuration] in the [Context] while value can be
//! accessed by their own key, namely [CONFIGURATION_COMMITS], [CONFIGURATION_USER], [CONFIGURATION_REPO],
//! [CONFIGURATION_LAST_TAG] and [CONFIGURATION_NEW_TAG].
//!
//! To access to the list of commits, one can write :
//!  - `context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_COMMITS.to_string()]`
//! This will return a [Value] enumeration which can be read by it's method [as_commits()].
//!
//! Shared datas are retrieved from a [Context] structure.
//! This context should contain a [CONFIGURATION_KEY] associated with its [Configuration] structure.
//! This [Configuration] should contain a [CONFIGURATION_COMMITS] to access the list of commits, [CONFIGURATION_USER]
//! to access the user, [CONFIGURATION_REPO] to access the repository URL, [CONFIGURATION_LAST_TAG] to access the last
//! tag of a repository and [CONFIGURATION_NEW_TAG] to access the new tag of a repository.
//!
//! [CONFIGURATION_COMMITS] is used by `sleppa_commit_analyzer` and `sleppa_changelog`.
//! [CONFIGURATION_USER] is used by `sleppa_changelog` and `sleppa_versioner`.
//! [CONFIGURATION_REPO] is used by `sleppa_changelog` and `sleppa_code_archiver`.
//! [CONFIGURATION_LAST_TAG] is used by `sleppa_changelog`, `sleppa_versioner` and `sleppa_code_archiver`.
//! [CONFIGURATION_NEW_TAG] is used by `sleppa_changelog`.

pub mod constants;
pub mod errors;

use constants::{
    CONFIGURATION_COMMITS, CONFIGURATION_KEY, CONFIGURATION_LAST_TAG, CONFIGURATION_NEW_TAG, CONFIGURATION_REPO,
    CONFIGURATION_USER,
};
use sleppa_primitives::{
    repositories::{github::GithubRepository, RepositoryTag, RepositoryUser},
    Commit, Value,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub map: HashMap<String, Value>,
}

/// The context structure is used to store configuration.
#[derive(Default)]
pub struct Context {
    pub configurations: HashMap<String, Configuration>,
}

impl Context {
    /// Loads an optionnal [RepositoryUser] from the context
    pub fn load_user(&self) -> Option<RepositoryUser> {
        self.configurations[&CONFIGURATION_KEY.to_string()].map[CONFIGURATION_USER].as_user()
    }

    /// Loads an optionnal [GithubRepository] from the context
    pub fn load_repository(&self) -> Option<GithubRepository> {
        self.configurations[&CONFIGURATION_KEY.to_string()].map[CONFIGURATION_REPO].as_repository()
    }

    /// Loads an optionnal last [RepositoryTag] of a repository from the context
    pub fn load_last_tag(&self) -> Option<RepositoryTag> {
        self.configurations[&CONFIGURATION_KEY.to_string()].map[CONFIGURATION_LAST_TAG].as_tag()
    }

    /// Loads an optionnal new [RepositoryTag] of a repository from the context
    pub fn load_new_tag(&self) -> Option<RepositoryTag> {
        self.configurations[&CONFIGURATION_KEY.to_string()].map[CONFIGURATION_NEW_TAG].as_tag()
    }

    /// Loads an optionnal Vec<[Commit]> from the context
    pub fn load_commits(&self) -> Option<Vec<Commit>> {
        self.configurations[&CONFIGURATION_KEY.to_string()].map[CONFIGURATION_COMMITS].as_commits()
    }
}

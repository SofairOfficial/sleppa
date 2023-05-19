//! Sleppa commit analyzer package
//!
//! This crate analyzes the content of commit messages and calculate a [semantic version](https://semver.org) number.
//! For doing so, the analyzer leverages on the [ReleaseRule]s defined in the [Configuration].
//!
//! In order to match a new [ReleaseAction], the commit messages from the last tag must be retrieved.
//!
//! As only one release action type must be defined for a new release, only the higher one is kept :
//! - Major > Minor > Patch
//!
//! The `analyze` function sets the correct release action field of the given [Commit]s if a [ReleaseAction] is found.

mod configuration;
mod errors;

use configuration::{errors::ConfigurationResult, *};
use errors::*;
use sleppa_primitives::{Commit, ReleaseAction};
use sleppa_primitives::{Configurable, Context};
use std::path::Path;

/// Defines the commit analyzer plugin
///
/// This plugins aims at analyzing given commits messages to determine the [ReleaseAction] type to
/// apply.
#[derive(Debug, Default)]
pub struct CommitAnalyzerPlugin;

impl CommitAnalyzerPlugin {
    /// Verifies multiple commit messages to retrieve the higher release action type to apply.
    ///
    /// This function receives a list of commit messages, as a vector of [String]s, and analyzes them
    /// to retrieve the release action type to apply since the last tag.
    /// As it is impossible to have two release action types at the same time, only the higher one is kept.
    /// Also the analyzed [Commit] is modified to provide the [ReleaseAction] found to it.
    pub fn run(&self, _context: &Context, commits: &mut Vec<Commit>, rules: &ReleaseRules) -> Option<ReleaseAction> {
        let mut major_count = 0;
        let mut minor_count = 0;
        let mut patch_count = 0;

        // Matches the release action type according to the commit message contents.
        for commit in commits {
            match self.execute(commit, rules) {
                Ok(ReleaseAction::Major) => {
                    major_count += 1;

                    // Sets the release action to the [Commit]
                    commit.release_action = Some(ReleaseAction::Major);
                }
                Ok(ReleaseAction::Minor) => {
                    minor_count += 1;

                    // Sets the release action to the [Commit]
                    commit.release_action = Some(ReleaseAction::Minor);
                }
                Ok(ReleaseAction::Patch) => {
                    patch_count += 1;

                    // Sets the release action to the [Commit]
                    commit.release_action = Some(ReleaseAction::Patch);
                }
                Err(_err) => continue,
            }
        }

        // Returns only the higher action release type.
        if major_count > 0 {
            Some(ReleaseAction::Major)
        } else if minor_count > 0 {
            Some(ReleaseAction::Minor)
        } else if patch_count > 0 {
            Some(ReleaseAction::Patch)
        } else {
            None
        }
    }

    /// Parses a message and matches a ReleaseAction.
    ///
    /// This function reads a given message and verifies if the message matches a [ReleaseAction].
    /// thanks to the trait [ReleaseRuleHandler].
    /// If no match is found, a [CommitAnalyzerError] is returned.
    fn execute(&self, commit: &Commit, release_rule: &ReleaseRules) -> CommitAnalyzerResult<ReleaseAction> {
        if release_rule[&ReleaseAction::Major].handle(commit).is_ok() {
            Ok(ReleaseAction::Major)
        } else if release_rule[&ReleaseAction::Minor].handle(commit).is_ok() {
            Ok(ReleaseAction::Minor)
        } else if release_rule[&ReleaseAction::Patch].handle(commit).is_ok() {
            Ok(ReleaseAction::Patch)
        } else {
            Err(CommitAnalyzerError::ErrorNoMatching())
        }
    }
}

impl Configurable<&Path, ConfigurationResult<Configuration>> for CommitAnalyzerPlugin {
    /// Loads the configuration file for the CommitAnalyzerPlugin
    ///
    /// The [CommitAnalyzerPlugin] needs a [Configuration] in order to run. This Configuration
    /// is loaded thanks to a given file path.
    fn load(&self, input: &Path) -> ConfigurationResult<Configuration> {
        try_parse(input)
    }
}

#[cfg(test)]
mod tests;

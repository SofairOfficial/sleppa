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
//!
//! Datas used to analyze commits are retrieved from a [Context] structure.
//! This [Context] should contain a [CONTEXT_COMMITS] to access the list of commits to analyze.
//!
//! User could defined it's own [ReleaseRules] to analyze commits by providing a configuration thanks to
//! the method [with_configuration(&mut self, configuration_file_path: &str)].
//! If no file path is provided, the default [ReleaseRules] are used.

mod configuration;
mod errors;

use configuration::{try_parse, CommitAnalyzerConfiguration, ReleaseRuleHandler, ReleaseRules};
use errors::{CommitAnalyzerError, CommitAnalyzerResult};
use sleppa_primitives::{
    constants::CONTEXT_COMMITS, repositories::GitRepository, Commit, Context, ReleaseAction, Value,
};
use std::path::Path;

/// Defines the commit analyzer plugin
///
/// This plugins aims at analyzing given commits messages to determine the [ReleaseAction] type to
/// apply.
#[derive(Debug, Default)]
pub struct CommitAnalyzerPlugin {
    configuration: CommitAnalyzerConfiguration,
}

impl CommitAnalyzerPlugin {
    /// Implements the creation of a new `CommitAnalyzerPlugin`
    ///
    /// This loads the defaults [ReleaseRules] for the configuration.
    pub fn new() -> Self {
        Self {
            configuration: CommitAnalyzerConfiguration::default(),
        }
    }

    /// Loads the configuration file for the CommitAnalyzerPlugin
    ///
    /// If the user wants others [ReleaseRules], then [CommitAnalyzerPlugin] needs another configuration in order to run.
    /// These new rules are provided by reading a given file thanks to its `configuration_file_path`.
    pub fn with_configuration(&mut self, configuration_file_path: &str) -> CommitAnalyzerResult<&mut Self> {
        let path = Path::new(configuration_file_path);
        self.configuration = try_parse(path)?;

        Ok(self)
    }

    /// Verifies multiple commit messages to retrieve the higher release action type to apply.
    ///
    /// This function loads the commits from the [Context], and analyzes them to retrieve the release action
    /// type to apply since the last tag.
    /// As it is impossible to have two release action types at the same time, only the higher one is kept.
    /// Also the analyzed [Commit] is updated to provide the [ReleaseAction] associated to it.
    pub fn run<R: GitRepository>(&self, context: &mut Context<R>) -> CommitAnalyzerResult<Option<ReleaseAction>> {
        let mut major_count = 0;
        let mut minor_count = 0;
        let mut patch_count = 0;

        let mut commits = match context.load_commits() {
            Some(value) => value,
            None => return Err(CommitAnalyzerError::InvalidContext("No commits found.".to_string())),
        };

        let rules = &self.configuration.release_rules;

        // Matches the release action type according to the commit message contents.
        for commit in &mut commits {
            match self.execute(commit, rules) {
                Ok(ReleaseAction::Major) => {
                    major_count += 1;

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
        context.map.insert(CONTEXT_COMMITS.to_string(), Value::Commits(commits));

        // Returns only the higher action release type.
        if major_count > 0 {
            Ok(Some(ReleaseAction::Major))
        } else if minor_count > 0 {
            Ok(Some(ReleaseAction::Minor))
        } else if patch_count > 0 {
            Ok(Some(ReleaseAction::Patch))
        } else {
            Ok(None)
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

#[cfg(test)]
mod tests;

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
//! This context should contain a [CONFIGURATION_KEY] associated with its [Configuration] structure.
//! This [Configuration] should contain a [CONFIGURATION_COMMITS] to access the list of commits to analyze.
//!
//! User could defined it's own [ReleaseRules] to analyze commits by providing a configuration file.
//! This file could be accessed with the [COMMIT_ANALYZER_KEY] to access the [COMMIT_ANALYZER_FILE]. If these keys are
//! not defined, the process fallback to the default [ReleaseRules].

mod configuration;
pub mod constants;
mod errors;

use configuration::{try_parse, CommitAnalyzerConfiguration, ReleaseRuleHandler, ReleaseRules};
use constants::{COMMIT_ANALYZER_FILE, COMMIT_ANALYZER_KEY};
use errors::{CommitAnalyzerError, CommitAnalyzerResult};
use sleppa_configuration::{
    constants::{CONFIGURATION_COMMITS, CONFIGURATION_KEY},
    Context,
};
use sleppa_primitives::{Commit, ReleaseAction, Value};
use std::path::Path;

/// Defines the commit analyzer plugin
///
/// This plugins aims at analyzing given commits messages to determine the [ReleaseAction] type to
/// apply.
#[derive(Debug)]
pub struct CommitAnalyzerPlugin {
    configuration: CommitAnalyzerConfiguration,
}

impl CommitAnalyzerPlugin {
    pub fn build(context: &Context) -> CommitAnalyzerResult<Self> {
        let plugin = match CommitAnalyzerPlugin::load_file(context) {
            Ok(value) => CommitAnalyzerPlugin {
                configuration: try_parse(Path::new(&value))?,
            },
            Err(_) => CommitAnalyzerPlugin {
                configuration: CommitAnalyzerConfiguration::default(),
            },
        };
        Ok(plugin)
    }

    /// Verifies multiple commit messages to retrieve the higher release action type to apply.
    ///
    /// This function receives a list of commit messages, as a vector of [String]s, and analyzes them
    /// to retrieve the release action type to apply since the last tag.
    /// As it is impossible to have two release action types at the same time, only the higher one is kept.
    /// Also the analyzed [Commit] is modified to provide the [ReleaseAction] found to it.
    pub fn run(&self, context: &mut Context) -> CommitAnalyzerResult<Option<ReleaseAction>> {
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
        context.configurations.get_mut(CONFIGURATION_KEY).map(|config| {
            config
                .map
                .insert(CONFIGURATION_COMMITS.to_string(), Value::Commits(commits))
        });

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

    /// Loads the configuration file for the CommitAnalyzerPlugin
    ///
    /// If the user wants others [ReleaseRules], then [CommitAnalyzerPlugin] needs a [Configuration] in order to run.
    /// The path of the file is accessed with thanks [COMMIT_ANALYZER_KEY] in the [Context] and the [COMMIT_ANALYZER_FILE]
    /// in the associated Configuration.
    /// The default rules are loaded if no path is provided.
    fn load_file(context: &Context) -> CommitAnalyzerResult<String> {
        let file = match context
            .configurations
            .get(COMMIT_ANALYZER_KEY)
            .and_then(|conf| conf.map.get(COMMIT_ANALYZER_FILE))
            .and_then(|data| data.as_string())
        {
            Some(value) => value,
            None => {
                return Err(CommitAnalyzerError::InvalidContext(
                    "No value found, loads the default release rules".to_string(),
                ));
            }
        };

        Ok(file.to_string())
    }
}

#[cfg(test)]
mod tests;

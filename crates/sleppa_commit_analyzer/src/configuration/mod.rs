//! Sleppa configuration management package
//!
//! This parser reads the configuration file and converts it to Rust structure [Configuration].
//! This configuration file must contain a `[release_rule]` section with three types of release actions, namely `major`,
//! `minor` and `patch`.
//! These three release action types are mandatory and must be written in lower case, as shown below :
//!
//!```toml
//! [release_rules]
//! major = { format = "regex", grammar = '^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! minor = { format = "regex", grammar = '^(?P<type>build|ci|docs|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! patch = { format = "regex", grammar = '^(?P<type>fix|perf|refac|sec|style|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//!```
//!
//! These are the default release action types used by `sleppa_commit_analyzer`, as described in the contributor's bible.
//!
//! For each release rule, user must define a format and a grammar. The format defines the idiom used for describing
//! the grammar that will be used for analysing a commit message. Two formats are now supported,
//! namely `regex` (for [regular expression](https://en.wikipedia.org/wiki/Regular_expression))
//! and `peg` (for [parsing expression grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar)).
//!
//! The function [try_parse] returns a [CommitAnalyzerConfiguration] :
//! - `Hashmap<ReleaseAction, ReleaseRule { ReleaseRuleFormat, String }>`
//!
//! The trait [ReleaseRuleHandler] handles the release rule and verifies if a commit message
//! matches a grammar.

pub mod errors;

use errors::{ConfigurationError, ConfigurationResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sleppa_primitives::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration data structure
///
/// This structure will be used to deserialize the toml into this Rust usable type.
///
/// The `release_rules` hashmap contains 3 keys : `major`, `minor` and `patch`.
/// For every key a [ReleaseRule] is associated.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitAnalyzerConfiguration {
    pub release_rules: ReleaseRules,
}

/// Enumerates available format for a release rule.
///
/// Two format are available : Regex and PEG.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseRuleFormat {
    /// Grammar of the release rule is defined as a [regular expression](https://en.wikipedia.org/wiki/Regular_expression)
    Regex,
    /// Grammar of the release rule is defined using parsing expression grammar [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)
    Peg,
}

/// Release rule ressource
///
/// A ReleaseRule is defined by its format as a [ReleaseRuleFormat] and its associated
/// grammar as a [String].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseRule {
    /// The format is a [ReleaseRuleFormat] : `Regex` or `Peg`
    pub format: ReleaseRuleFormat,
    /// Expression used to analyze the commit message
    pub grammar: String,
}

/// Type alias used for typing release rule
pub type ReleaseRules = HashMap<ReleaseAction, ReleaseRule>;

/// A handler to match the commit message to a release rule grammar.
pub trait ReleaseRuleHandler {
    /// Verifies if a commit message matches a [ReleaseRule] grammar.
    fn handle(&self, commit: &Commit) -> ConfigurationResult<()>;
}

impl Default for CommitAnalyzerConfiguration {
    /// Loads a default CommitAnalyzerConfiguration
    ///
    /// This default [CommitAnalyzerConfiguration] is the one used by Sleppa to parse commit's message
    fn default() -> Self {
        let mut releaserules: HashMap<ReleaseAction, ReleaseRule> = HashMap::new();
        releaserules.insert(
            ReleaseAction::Major,
            ReleaseRule {
                format: ReleaseRuleFormat::Regex,
                grammar: r"^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
            },
        );
        releaserules.insert(
            ReleaseAction::Minor,
            ReleaseRule {
                format: ReleaseRuleFormat::Regex,
                grammar: r"^(?P<type>build|ci|docs|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
            },
        );
        releaserules.insert(
            ReleaseAction::Patch,
            ReleaseRule {
                format: ReleaseRuleFormat::Regex,
                grammar: r"^(?P<type>fix|perf|refac|sec|style|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
            },
        );

        CommitAnalyzerConfiguration {
            release_rules: releaserules,
        }
    }
}

impl ReleaseRuleHandler for ReleaseRule {
    /// Verifies if a commit message matches a release rule grammar.
    ///
    /// The given commit message is parsed using the release rule's grammar and
    /// in case it matches, an Ok(()) is returned.
    fn handle(&self, commit: &Commit) -> ConfigurationResult<()> {
        match &self.format {
            ReleaseRuleFormat::Regex => {
                let regex = match Regex::new(self.grammar.as_str()) {
                    Ok(regex) => regex,
                    Err(err) => {
                        return Err(ConfigurationError::RegexError(err));
                    }
                };
                match regex.captures(commit.message.as_str()) {
                    Some(_cap) => Ok(()),
                    None => Err(ConfigurationError::ErrorNoMatch()),
                }
            }
            ReleaseRuleFormat::Peg => {
                unimplemented!()
            }
        }
    }
}

/// Loads a configuration file given a file path name.
///
/// The given toml configuration file is loaded and parsed, and if successful,
/// a [Configuration] is returned or a [ConfigurationError] otherwise.
/// The parsing returns a [ConfigurationError] if a [ReleaseAction] is missing or if the
/// `format` is not recognized.
pub(crate) fn try_parse(path: &Path) -> ConfigurationResult<CommitAnalyzerConfiguration> {
    let content = fs::read_to_string(path)?;

    let config: CommitAnalyzerConfiguration = toml::from_str(&content)?;

    // Verify that the configuration file contains a release rule for each release action types.
    if config.release_rules.get(&ReleaseAction::Major).is_none() {
        return Err(ConfigurationError::IncorrectReleaseAction(
            "major is missing".to_string(),
        ));
    } else if config.release_rules.get(&ReleaseAction::Minor).is_none() {
        return Err(ConfigurationError::IncorrectReleaseAction(
            "minor is missing".to_string(),
        ));
    } else if config.release_rules.get(&ReleaseAction::Patch).is_none() {
        return Err(ConfigurationError::IncorrectReleaseAction(
            "patch is missing".to_string(),
        ));
    }

    Ok(config)
}

#[cfg(test)]
mod tests;

//! Sleppa configuration management package
//!
//! This parser reads the configuration file and converts it to Rust structure [Configuration].
//! This configuration file must contain a `[release_rule]` section with three types of release actions, namely `major`, `minor` and `patch`.
//! These three release action types are mandatory and must be written in lower case, as shown in the example below :
//!
//!```toml
//! [release_rules]
//! major = { format = "regex", grammar = '^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! minor = { format = "regex", grammar = '^(?P<type>ci|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! patch = { format = "regex", grammar = '^(?P<type>fix|refac|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//!```
//!
//! For each release rule, user must define a format and a grammar. The format defines the idiom used for describing
//! the grammar that will be used for analysing a commit message. Two formats are now supported,
//! namely `regex` (for [regular expression](https://en.wikipedia.org/wiki/Regular_expression))
//! and `peg` (for [parsing expression grammar](https://en.wikipedia.org/wiki/Parsing_expression_grammar)).
//!
//! The function [try_parse] returns a [Configuration] :
//! - `Hashmap<ReleaseAction, ReleaseRule { ReleaseRuleFormat, String }>`
//!
//! The trait [ReleaseRuleHandler] handles the release rule and verifies if a commit message
//! matches a grammar.

mod error;

use error::{ConfigurationError, ConfigurationResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Enumerates available release actions.
#[derive(PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseAction {
    /// Major release implying the left digit of a semantic version to be incremented (e.g. from `1.0.1` -> `2.0.0`)
    Major,
    /// Minor release implying the middle digit of a semantic version to be incremented (e.g. from `1.0.1` -> `1.1.0`)
    Minor,
    /// Patch release implying the right digit of a semantic version to be incremented (e.g. from `1.0.1` -> `1.0.2`)
    Patch,
}

/// Enumerates available format for a release rule.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseRuleFormat {
    /// Grammar of the release rule is defined as a regular expression
    Regex,
    /// Grammar of the release rule is defined using parsing expression grammar (PEG)
    Peg,
}

/// Type alias used for typing release rule
pub type ReleaseRules = HashMap<ReleaseAction, ReleaseRule>;

/// Configuration data structure
///
/// This structure will be used to deserialize the toml into this Rust usable type.
///
/// The `release_rules` hashmap contains 3 keys : `major`, `minor` and `patch`.
/// For every key a [ReleaseRule] is associated.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configuration {
    pub release_rules: ReleaseRules,
}

/// Implementation of the `new` method : `Configuration::new()`
impl Configuration {
    pub fn new() -> Self {
        Configuration::default()
    }
}

/// Release rule ressource
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseRule {
    /// The format is `Regex` or `Peg`
    pub format: ReleaseRuleFormat,
    /// Expression used to analyze the commit message
    pub grammar: String,
}

pub trait ReleaseRuleHandler {
    /// Verifies if a commit message matches a release rule grammar.
    ///
    /// The given commit message is parsed using the release rule's grammar and
    /// in case it matches, an Ok(()) is returned.
    fn handle(&self, message: &str) -> ConfigurationResult<()>;
}

impl ReleaseRuleHandler for ReleaseRule {
    fn handle(&self, message: &str) -> ConfigurationResult<()> {
        match &self.format {
            ReleaseRuleFormat::Regex => {
                let regex = match Regex::new(self.grammar.as_str()) {
                    Ok(regex) => regex,
                    Err(err) => {
                        return Err(ConfigurationError::RegexError(err));
                    }
                };
                match regex.captures(message) {
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
pub fn try_parse(path: &Path) -> ConfigurationResult<Configuration> {
    let content = fs::read_to_string(path)?;

    let config: Configuration = toml::from_str(&content)?;

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

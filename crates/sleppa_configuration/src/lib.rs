//! # Configuration file parser
//!
//! Sleppa configuration parser reads the `sleppa.toml` configuration file and deserializes it.
//!
//! This parser reads the `sleppa.toml` configuration file and converts it to Rust structure [Configuration].
//! This configuration file should contain a `[release_rule]` section with the 3 release types: `major`, `minor` and `patch`.
//! These release types are mandatory and are case sensitive. Here an exemple of such a file :
//!
//!```toml
//! [release_rules]
//! major = { format = "regex", grammar = '^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! minor = { format = "regex", grammar = '^(?P<type>ci|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! patch = { format = "regex", grammar = '^(?P<type>fix|refac|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//!```
//!
//! For each release type, user must define a `format` to read the associated `grammar`.
//!
//! The supported format are defined by an enum [ReleaseRuleFormat] :
//! - Regex: declared by `regex`
//! - Peg: declared by `peg`
//!
//! The function [try_parse] returns a [Configuration] :
//! - `Hashmap<ReleaseAction, ReleaseRuleDefinition { ReleaseRuleFormat, String }>`
//!
//! The trait [ReleaseRuleHandler] handles the release rule definition and verifies if a message
//! matches a [ReleaseAction].
//!

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
    /// Major release incrementing the first number : `1.0.1` -> `2.0.0`
    Major,
    /// Minor release incrementing the second number : `1.0.1` -> `1.1.0`
    Minor,
    /// Patch release incrementing the third number : `1.0.1` -> `1.0.2`
    Patch,
}

/// Enumerates available format for a release rule.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseRuleFormat {
    /// Format using Regex syntax
    Regex,
    /// Format using Peg syntax
    Peg,
}

/// The new type associated with the `release_rules` section in the `sleppa.toml`.
pub type ReleaseRules = HashMap<ReleaseAction, ReleaseRuleDefinition>;

/// This structure will be used to deserialize the toml into this Rust usable type.
///
/// The `release_rules` hashmap contains 3 keys : `major`, `minor` and `patch`.
/// For every key a [ReleaseRuleDefinition] is associated.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configuration {
    pub release_rules: ReleaseRules,
}

/// Implementation of the `new` method : `Configuration::new()`
impl Configuration {
    pub fn new() -> Self {
        Configuration {
            release_rules: HashMap::new(),
        }
    }
}

/// The ReleaseRuleDefinition contains a grammar and the format associated to parse it.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseRuleDefinition {
    /// The format is `Regex` or `Peg`
    pub format: ReleaseRuleFormat,
    pub grammar: String,
}

/// Trait to analyze a `message` and if it matches a [ReleaseRules].
pub trait ReleaseRuleHandler {
    /// Reads a message, compares it to a `grammar` in a `format` (Regex or Peg)
    /// and returns a boolean. True if there is a match , false otherwise.
    fn handle(&self, message: &str) -> ConfigurationResult<bool>;
}

/// Implementation of the trait [ReleaseRuleHandler] for [ReleaseRuleDefinition].
impl ReleaseRuleHandler for ReleaseRuleDefinition {
    fn handle(&self, message: &str) -> ConfigurationResult<bool> {
        match &self.format {
            ReleaseRuleFormat::Regex => {
                let re = match Regex::new(self.grammar.as_str()) {
                    Ok(re) => re,
                    Err(err) => {
                        return Err(ConfigurationError::RegexError(err));
                    }
                };
                let captured = match re.captures(message) {
                    Some(_cap) => true,
                    None => false,
                };
                Ok(captured)
            }
            ReleaseRuleFormat::Peg => {
                todo!()
            }
        }
    }
}

/// Tries parsing the `sleppa.toml` configuration file and returns a [Configuration] or
/// a [ConfigurationError].
///
/// The `path` is the path to the configuration file `sleppa.toml`.
/// The parsing returns a [ConfigurationError] if a [ReleaseAction] is missing or if the `format` is not recognized.
pub fn try_parse(path: &Path) -> ConfigurationResult<Configuration> {
    let content = fs::read_to_string(path)?;

    let config: Configuration = toml::from_str(&content)?;

    // Verify that `sleppa.toml` contains a release rule for each release action types.
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

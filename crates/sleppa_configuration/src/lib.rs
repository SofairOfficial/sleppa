//! # Configuration file parser
//!
//! Sleppa configuration parser reads the `sleppa.toml` configuration file and deserializes it.
//!
//! This parser reads the `sleppa.toml` configuration file and convert it to Rust structure [`Configuration`].
//! This configuration file should contain a `[release_rule]` section with the 3 release types: `major`, `minor` and `patch`.
//! These release types are mandatory and are case sensitive. Here an exemple of such a file :
//!
//! ```toml
//! [release_rules]
//! major = { format = "regex", grammar = '^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! minor = { format = "regex", grammar = '^(?P<type>ci|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//! patch = { format = "regex", grammar = '^(?P<type>fix|refac|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }
//!```
//!
//!For each release type, user must define a `format` to read the associated `grammar`.
//!
//!The supported format are defined by an enum [`ReleaseRuleFormat`] :
//! - Regex: declared by `regex`
//! - Peg: declared by `peg`
//!
//! The function `try_parse_sleppatoml` will return a [`Configuration`] :
//! - `Hashmap<ReleaseAction, ReleaseRule { ReleaseRuleFormat, String }>`
//!

mod error;

use error::{ConfigurationError, ConfigurationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Enumerates available release actions
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

/// Enumerates available format for a release rule
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseRuleFormat {
    /// Format using Regex syntax
    Regex,
    /// Format using Peg syntax
    Peg,
}

/// `Configuration` structure
///
/// This structure will be used to deserialize the toml into this Rust usable
/// type.
/// The `release_rules` hashmap contains 3 keys : `major`, `minor` and `patch`.
/// For every key a [`ReleaseRule`] is associated.
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration<F> {
    pub release_rules: HashMap<ReleaseAction, ReleaseRule<F>>,
}

/// Implementation of the `new` method : `Configuration::new()`
impl Configuration<ReleaseRuleFormat> {
    pub fn new() -> Self {
        Configuration {
            release_rules: HashMap::new(),
        }
    }
}

/// Implementation of the `default` method : `Configuration::default()`
impl Default for Configuration<ReleaseRuleFormat> {
    fn default() -> Self {
        Self::new()
    }
}

/// `ReleaseRule` structure
///
/// The ReleaseRule defined a grammar and the format associated to analyze it.
/// It is generic over F to implement any convenient format.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseRule<F> {
    pub format: F,
    pub grammar: String,
}

/// `ReleaseRuleValidator` trait
///
/// Trait to implement to to validate the [`ReleaseRule`] over a generic `format` F.
pub trait ReleaseRuleValidator {
    type F;
    /// The `validate` function reads a message, compare it to a `grammar` in a `format` F
    /// and return a [`ReleaseAction`] if found.
    fn validate<F>(&self, message: String) -> Option<ReleaseAction>;
}

/// try_parser_sleppatoml function
///
/// Try to parse the `sleppa.toml` configuration file and return a [`Configuration`] structure or
/// an `Error`
pub fn try_parse_sleppatoml(path: &Path) -> ConfigurationResult<Configuration<ReleaseRuleFormat>> {
    let reading = fs::read_to_string(path)?;

    let config: Configuration<ReleaseRuleFormat> = toml::from_str(&reading)?;

    println!("{:?}", config);

    // Verify the `sleppa.toml` contains the exact 3 [`ReleaseAction`] : `Major`, `Minor` and `Patch`
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

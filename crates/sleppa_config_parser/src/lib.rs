//! Sleppa configuration parser reads the `sleppa.toml` configuration file and deserializes it.
//!
//! This parser reads the `sleppa.toml` configuration file and convert it to rust structure `Config`.
//! This configuration file should contain a `[release_rule]` section with the 3 release types: `major`, `minor` and `patch`.
//! These release types are mandatory and are case sensitive. Here an exemple of such a file :
//!
//! ```
//! [release_rules]
//! major = { keywords = ["default"] }
//! minor = { keywords = [
//!    "build",
//!    "ci",
//!    "feat",
//! ], commit_format = { parser = "regex", grammar = '^(?P<type>build|ci|feat){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' } }
//! patch = { keywords = [
//!    "fix",
//!    "perf",
//! ], commit_format = { parser = "peg", grammar = '^(?P<type>fix|perf){1}:\s.*[a-z0-9]$' } }
//!```
//!
//!For each release type, user can define the behavior of the semantic release with a custom parser.
//!
//!The `keyword` section is mandatory and could be of three types :
//! - "default" : release rules of Sofair project will be used.
//! - "angular" : release rules of Angular project will be used.
//! - array of strings e.g. `["build", "feat", "ci"] : these are the keywords used inside the commit's message
//! (like `feat: add a function`). In this case the `commit_format` section must be define with a `parser` (`"regex"`
//! or `"peg"` only) and a grammar. The grammar should contain the keywords.
//!
//! The function `try_parse_sleppatoml` will return a `Config` :
//! - Hashmap<ReleaseAction, ReleaseRule {
//!     Vec<String>, Option<ReleaseRuleFormat {
//!         Parser, String
//!     }>
//!   }>
//!

use error::{ConfparserError, ConfparserResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

mod error;

/// Declare the 3 possible release types.
#[derive(PartialEq, Debug, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseAction {
    Patch,
    Minor,
    Major,
}

/// Declare the 2 possible parsers type used for custom configuration.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Parser {
    Regex,
    Peg,
}

/// The Config structure returned by the TOML parsing function.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub release_rules: HashMap<ReleaseAction, ReleaseRule>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            release_rules: HashMap::new(),
        }
    }
}

/// The ReleaseRule is defined by the keywords inside the `sleppa.toml`
/// and the `commit_format` for a custom type.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseRule {
    pub keywords: Vec<String>,
    pub commit_format: Option<ReleaseRuleFormat>,
}

impl ReleaseRule {
    /// This method verify what keywords have been provided. If the `default` or `angular`
    /// are present, then loads the release rules associated and provide them to `Config`
    fn verify_release_rule(&mut self) {
        if &self.keywords[0] == "default" {
            self.commit_format = Some(ReleaseRuleFormat::default());
        } else if &self.keywords[0] == "angular" {
            self.commit_format = Some(ReleaseRuleFormat::angular());
        } else {
            // For custom commit's type keyword with a custom parser
            todo!()
        }
    }
}

/// The format of the release rules used by the parser.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseRuleFormat {
    pub parser: Parser,
    pub grammar: String,
}

impl ReleaseRuleFormat {
    /// Implement the Sofair parser (the default one) with the associated keywords
    /// This grammar is used for Major, Minor and Patch
    pub fn default() -> Self {
        ReleaseRuleFormat {
            parser: Parser::Regex,
            grammar: r"^(?P<type>break|build|ci|docs|feat|fix|perf|refac|sec|style|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        }
    }

    /// Implement the Angular parser with the associated keywords
    /// This grammar is used for Major, Minor and Patch
    pub fn angular() -> Self {
        todo!()
    }

    // Get the grammar of a ReleaseRuleFormat
    pub fn get_grammar(&self) -> String {
        return self.grammar.to_owned();
    }

    // Get the parser of a ReleaseRuleFormat
    pub fn get_parser(&self) -> Parser {
        return self.parser.to_owned();
    }
}

// Try to parse the sleppa.toml configuration file and return a `Config`
pub fn try_parse_sleppatoml(path: &Path) -> ConfparserResult<Config> {
    let reading = fs::read_to_string(path)?;

    let mut config: Config = toml::from_str(&reading)?;

    println!("{:?}", config);

    // Verify the sleppa.toml contains the exact 3 release types : MAJOR, MINOR and PATCH
    if config.release_rules.get(&ReleaseAction::Major).is_none() {
        return Err(ConfparserError::IncorrectReleaseAction("MAJOR is missing".to_string()));
    } else if config.release_rules.get(&ReleaseAction::Minor).is_none() {
        return Err(ConfparserError::IncorrectReleaseAction("MINOR is missing".to_string()));
    } else if config.release_rules.get(&ReleaseAction::Patch).is_none() {
        return Err(ConfparserError::IncorrectReleaseAction("PATCH is missing".to_string()));
    }

    config
        .release_rules
        .get_mut(&ReleaseAction::Major)
        .map(|conf| conf.verify_release_rule());
    config
        .release_rules
        .get_mut(&ReleaseAction::Minor)
        .map(|conf| conf.verify_release_rule());
    config
        .release_rules
        .get_mut(&ReleaseAction::Patch)
        .map(|conf| conf.verify_release_rule());

    return Ok(config);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn ok_parser_sleppatoml() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("sleppa.toml");
        let mut sleppatoml = File::create(&file_path).unwrap();

        writeln!(&mut sleppatoml, "[release_rules]").unwrap();
        writeln!(&mut sleppatoml, r#"major = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"minor = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"patch = {{ keywords = [ "default" ] }}"#).unwrap();

        let config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
        assert_eq!(
            config.release_rules[&ReleaseAction::Major].keywords,
            ["default".to_string()]
        );
        assert_eq!(
            config.release_rules[&ReleaseAction::Minor].keywords,
            ["default".to_string()]
        );
        assert_eq!(
            config.release_rules[&ReleaseAction::Patch].keywords,
            ["default".to_string()]
        );
        assert_eq!(
            config.release_rules[&ReleaseAction::Major]
                .commit_format
                .as_ref()
                .unwrap()
                .get_parser(),
            Parser::Regex
        );

        assert_eq!(
            config.release_rules[&ReleaseAction::Major]
                .commit_format
                .as_ref()
                .unwrap()
                .get_grammar(),
            r"^(?P<type>break|build|ci|docs|feat|fix|perf|refac|sec|style|test){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$"
                .to_string()
        );
    }

    #[test]
    #[should_panic(expected = "unknown variant `Major`, expected one of `patch`, `minor`, `major")]
    fn fail_case_sensitive_sleppatoml() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("sleppa.toml");
        let mut sleppatoml = File::create(&file_path).unwrap();

        // The `sleppa.toml` file is case sensitive over "major", "minor" and "patch".
        writeln!(&mut sleppatoml, "[release_rules]").unwrap();
        writeln!(&mut sleppatoml, r#"Major = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"minor = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"patch = {{ keywords = [ "default" ] }}"#).unwrap();

        let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
    }

    #[test]
    #[should_panic(expected = "PATCH is missing")]
    fn fail_missing_release_sleppatoml() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("sleppa.toml");
        let mut sleppatoml = File::create(&file_path).unwrap();

        writeln!(&mut sleppatoml, "[release_rules]").unwrap();
        writeln!(&mut sleppatoml, r#"major = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"minor = {{ keywords = [ "default" ] }}"#).unwrap();

        let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
    }

    #[test]
    #[should_panic(expected = "missing field `release_rules`")]
    fn fail_missing_field_sleppatoml() {
        let tmp_dir = tempdir().unwrap();
        let file_path = tmp_dir.path().join("sleppa.toml");
        let mut sleppatoml = File::create(&file_path).unwrap();

        writeln!(&mut sleppatoml, r#"major = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"minor = {{ keywords = [ "default" ] }}"#).unwrap();
        writeln!(&mut sleppatoml, r#"patch = {{ keywords = [ "default" ] }}"#).unwrap();

        let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
    }
}

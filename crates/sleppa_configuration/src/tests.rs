//! Unit tests
//!
//! This testing module implements the unit tests for testing the configuration processing routines.

use super::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
/// Tests the function `try_parse` for a correct configuration file.
fn test_can_parse_configuration_file() -> TestResult<()> {
    // Creates a temporary directory and a temporary file.
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut file = File::create(&file_path)?;

    // Unit test preparation
    // Builds a correct configuration file for testing purpose.
    writeln!(&mut file, "[release_rules]")?;
    writeln!(
        &mut file,
        r#"major = {{ format = "regex" , grammar = '^(?P<type>break){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"minor = {{ format = "regex" , grammar = '^(?P<type>feat|refac){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;

    // Execution step
    let config = try_parse(&file_path)?;

    // Asserts the results of the function are correct.
    assert_eq!(
        config.release_rules[&ReleaseAction::Major].format,
        ReleaseRuleFormat::Regex
    );

    assert_eq!(
        config.release_rules[&ReleaseAction::Minor].format,
        ReleaseRuleFormat::Regex
    );

    assert_eq!(
        config.release_rules[&ReleaseAction::Patch].format,
        ReleaseRuleFormat::Regex
    );

    assert_eq!(
        config.release_rules[&ReleaseAction::Major].grammar,
        r"^(?P<type>break){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string()
    );

    assert_eq!(
        config.release_rules[&ReleaseAction::Minor].grammar,
        r"^(?P<type>feat|refac){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string()
    );

    assert_eq!(
        config.release_rules[&ReleaseAction::Patch].grammar,
        r"^(?P<type>fix){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string()
    );
    Ok(())
}

#[test]
/// Tests the `try_parse` function with an incorrect format in the configuration file.
fn test_fail_wrong_format() -> TestResult<()> {
    // Creates a temporary directory and a temporary file.
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut file = File::create(&file_path)?;

    // Unit test preparation
    // Builds an incorrect configuration file with a bad format.
    writeln!(&mut file, "[release_rules]")?;
    writeln!(
        &mut file,
        r#"major = {{ format = "rege" , grammar = '^(?P<type>break){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;

    // Asserts the result is an error: `rege` is not accepted.
    assert!(try_parse(&file_path).is_err());

    Ok(())
}

#[test]
/// Tests the `try_parse` function with an incorrect release type in the configuration file.
fn test_fail_case_sensitive() -> TestResult<()> {
    // Creates a temporary directory and a temporary file.
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut file = File::create(&file_path)?;

    // Unit test preparation
    // Builds an incorrect configuration file with a capital letter on the release action.
    writeln!(&mut file, "[release_rules]")?;
    writeln!(
        &mut file,
        r#"Major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;

    // Asserts the result is an error : `Major` is not accepted.
    assert!(try_parse(&file_path).is_err());
    Ok(())
}

#[test]
/// Tests the `try_parse` function with a missing release action in the configuration file
fn test_fail_missing_release() -> TestResult<()> {
    // Creates a temporary directory and a temporary file.
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut file = File::create(&file_path)?;

    // Unit test preparation
    // Builds an incorrect configuration file with a missing release action.
    writeln!(&mut file, "[release_rules]")?;
    writeln!(
        &mut file,
        r#"major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;

    // Asserts the result is an error : `patch` is missing.
    assert!(try_parse(&file_path).is_err());
    Ok(())
}

#[test]
/// Tests the `try_parse` function with the missing [release_rules] field.
fn test_fail_missing_field() -> TestResult<()> {
    // Creates a temporary directory and a temporary file.
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut file = File::create(&file_path)?;

    // Unit test preparation
    // Builds an incorrect configuration file with a missing field.
    writeln!(
        &mut file,
        r#"major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;
    writeln!(
        &mut file,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )?;

    // Asserts the result is an error : `[release_rules]` is missing.
    assert!(try_parse(&file_path).is_err());
    Ok(())
}

#[test]
/// Tests the `handle`function implementation for a ReleaseRule
fn test_can_trait_implementation_regex() {
    // Unit test preparation
    // Creates an instance of a ReleaseRule
    let release_rule_def = ReleaseRule {
        format: ReleaseRuleFormat::Regex,
        grammar: r"^(?P<type>feat|ci){1}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
    };

    // Creates correct messages for the grammar defined above
    // `feat` without scope is correct
    let msg0 = "feat: add a function";
    // `feat` with a scope is correct
    let msg1 = "feat(sync): add a function";
    // `ci` is correct
    let msg2 = "ci: add a workflow";

    // Creates incorrect messages for the grammar defined above
    // `break` is incorrect
    let msg3 = "break(sync): add a function";
    // Space after `feat` is incorrect
    let msg4 = "feat (sync): add a function";
    // No semi-column after `feat` is incorrect
    let msg5 = "feat add a function";

    // Asserts handle is matching
    assert!(release_rule_def.handle(msg0).is_ok());
    assert!(release_rule_def.handle(msg1).is_ok());
    assert!(release_rule_def.handle(msg2).is_ok());

    // Asserts handle is not matching
    assert!(release_rule_def.handle(msg3).is_err());
    assert!(release_rule_def.handle(msg4).is_err());
    assert!(release_rule_def.handle(msg5).is_err());
}

/// Unit test result type
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

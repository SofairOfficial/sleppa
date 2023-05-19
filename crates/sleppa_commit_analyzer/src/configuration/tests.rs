//! Unit tests
//!
//! This testing module implements the unit tests for testing the configuration processing routines.

use super::{errors::*, *};
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
/// Tests the parsing of a correct configuration file
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
/// Tests the parsing of a wrong `format` in a configuration file.
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
/// Tests the parsing of an incorrect release type in the configuration file.
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
/// Tests the parsing with a missing release action in the configuration file
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
/// Tests the parsing with the missing [release_rules] field.
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
/// Tests correct and incorrects messages to handle
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
    let commit0 = Commit::new(msg0.to_string(), "somehash".to_string());
    // `feat` with a scope is correct
    let msg1 = "feat(sync): add a function";
    let commit1 = Commit::new(msg1.to_string(), "somehash".to_string());
    // `ci` is correct
    let msg2 = "ci: add a workflow";
    let commit2 = Commit::new(msg2.to_string(), "somehash".to_string());

    // Creates incorrect messages for the grammar defined above
    // `break` is incorrect
    let msg3 = "break(sync): add a function";
    let commit3 = Commit::new(msg3.to_string(), "somehash".to_string());
    // Space after `feat` is incorrect
    let msg4 = "feat (sync): add a function";
    let commit4 = Commit::new(msg4.to_string(), "somehash".to_string());
    // No semi-column after `feat` is incorrect
    let msg5 = "feat add a function";
    let commit5 = Commit::new(msg5.to_string(), "somehash".to_string());

    // Asserts handle is matching
    assert!(release_rule_def.handle(&commit0).is_ok());
    assert!(release_rule_def.handle(&commit1).is_ok());
    assert!(release_rule_def.handle(&commit2).is_ok());

    // Asserts handle is not matching
    assert!(release_rule_def.handle(&commit3).is_err());
    assert!(release_rule_def.handle(&commit4).is_err());
    assert!(release_rule_def.handle(&commit5).is_err());
}

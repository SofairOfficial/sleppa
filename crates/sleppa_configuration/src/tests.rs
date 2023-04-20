//! # Unit tests
//!
//! This module contains all the unit tests of the sleppa_configuration crate.

use super::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_ok_parser_sleppatoml() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut sleppatoml = File::create(&file_path).unwrap();

    writeln!(&mut sleppatoml, "[release_rules]").unwrap();
    writeln!(
        &mut sleppatoml,
        r#"major = {{ format = "regex" , grammar = '^(?P<type>break){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"minor = {{ format = "regex" , grammar = '^(?P<type>feat|refac){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(?P<scope>\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();

    let config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
    assert_eq!(
        config.release_rules[&ReleaseAction::Major].format,
        ReleaseRuleFormat::Regex
    );

    let config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
    assert_eq!(
        config.release_rules[&ReleaseAction::Minor].format,
        ReleaseRuleFormat::Regex
    );
    let config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
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
}

#[test]
#[should_panic(expected = "unknown variant `Major`, expected one of `major`, `minor`, `patch`")]
fn test_fail_case_sensitive_sleppatoml() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut sleppatoml = File::create(&file_path).unwrap();

    // The `sleppa.toml` file is case sensitive over "major", "minor" and "patch".
    writeln!(&mut sleppatoml, "[release_rules]").unwrap();
    writeln!(
        &mut sleppatoml,
        r#"Major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();

    let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
}

#[test]
#[should_panic(expected = "patch is missing")]
fn test_fail_missing_release_sleppatoml() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut sleppatoml = File::create(&file_path).unwrap();

    writeln!(&mut sleppatoml, "[release_rules]").unwrap();
    writeln!(
        &mut sleppatoml,
        r#"major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();

    let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
}

#[test]
#[should_panic(expected = "missing field `release_rules`")]
fn test_fail_missing_field_sleppatoml() {
    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("sleppa.toml");
    let mut sleppatoml = File::create(&file_path).unwrap();

    writeln!(
        &mut sleppatoml,
        r#"major = {{ format = "regex" , grammar = '^(break){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"minor = {{ format = "regex" , grammar = '^(feat|refac){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();
    writeln!(
        &mut sleppatoml,
        r#"patch = {{ format = "regex" , grammar = '^(?P<type>fix){{1}}(\(\S.*\S\))?:\s.*[a-z0-9]$' }}"#
    )
    .unwrap();

    let _config = try_parse_sleppatoml(&file_path).expect("Failed to parse toml");
}

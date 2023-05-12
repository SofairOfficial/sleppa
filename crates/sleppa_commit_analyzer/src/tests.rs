//! Unit tests
//!
//! This testing module implements the unit tests for testing the commit analyzer routines.

use super::{errors::TestResult, *};

// Tests the function `execute`.
//
// The `execute` function must return a [ReleaseAction] if the message matches the defined regex.
// A [CommitAnalyzerError] is returned if no matches.
#[test]
fn test_can_execute() -> TestResult<()> {
    // Unit test preparation
    // Builds a correct [Configuration] structure for testing purpose.
    let mut config: Configuration = Configuration::new();
    config.release_rules.insert(
        ReleaseAction::Major,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(break){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );
    config.release_rules.insert(
        ReleaseAction::Minor,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(feat){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );
    config.release_rules.insert(
        ReleaseAction::Patch,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(refac){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );

    // Creates correct messages for the grammar defined above
    let correct_message_major_release_action = "break: add a function";
    let correct_message_patch_release_action = "refac: some ref";

    // Creates incorrect messages for the grammar defined above
    // "ci" doesn't refer to a release action type
    let incorrect_message_ci_not_match = "ci: some change";
    // No semi-column after the type
    let incorrect_message_no_semicolumn = "feat introduced new function";

    // Execution step
    let analyzer = CommitAnalyzerPlugin::default();

    // Asserts the results of the function match the correct ReleaseAction.
    assert_eq!(
        analyzer.execute(correct_message_major_release_action, &config.release_rules)?,
        ReleaseAction::Major
    );
    assert_eq!(
        analyzer.execute(correct_message_patch_release_action, &config.release_rules)?,
        ReleaseAction::Patch
    );

    // Asserts the results of the function are incorrects.
    assert!(analyzer
        .execute(incorrect_message_ci_not_match, &config.release_rules)
        .is_err());
    assert!(analyzer
        .execute(incorrect_message_no_semicolumn, &config.release_rules)
        .is_err());

    Ok(())
}

// Tests the function `analyze`.
//
// The `analyze` function analyzes multiple messages and return the highest
// [ReleaseAction] found from them.
// If a ReleaseAction is not found, a `None` is returned.
#[test]
fn test_can_analyze() {
    // Unit test preparation
    // Builds a correct [Configuration] structure for testing purpose.
    let mut config: Configuration = Configuration::new();
    config.release_rules.insert(
        ReleaseAction::Major,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(break){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );
    config.release_rules.insert(
        ReleaseAction::Minor,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(feat){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );
    config.release_rules.insert(
        ReleaseAction::Patch,
        ReleaseRule {
            format: ReleaseRuleFormat::Regex,
            grammar: r"^(refac){1}(\(\S.*\S\))?:\s.*[a-z0-9]$".to_string(),
        },
    );

    // Creates arrays of strings
    let correct_messages_major_release = vec![
        "break: add a function".to_string(),
        "refac: some ref".to_string(),
        "ci: some change".to_string(),
        "feat: a cool feature".to_string(),
    ];

    let correct_messages_patch_release = vec![
        "refac: documentation".to_string(),
        "refac: some ref".to_string(),
        "ci: some change".to_string(),
    ];

    let correct_no_release: Vec<String> = vec![];

    // Execution step
    let analyzer = CommitAnalyzerPlugin::default();

    // Asserts the results of the function matches the correct ReleaseAction
    assert_eq!(
        analyzer
            .analyze(correct_messages_major_release, &config.release_rules)
            .unwrap(),
        ReleaseAction::Major
    );
    assert_eq!(
        analyzer
            .analyze(correct_messages_patch_release, &config.release_rules)
            .unwrap(),
        ReleaseAction::Patch
    );
    assert!(analyzer.analyze(correct_no_release, &config.release_rules).is_none());
}

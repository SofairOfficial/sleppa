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
    let correct_message_major_release_action = "break: add a function".to_string();
    let correct_commit1 = Commit::new(correct_message_major_release_action, "somehash".to_string());

    let correct_message_patch_release_action = "refac: some ref".to_string();
    let correct_commit2 = Commit::new(correct_message_patch_release_action, "somehash".to_string());

    // Creates incorrect messages for the grammar defined above
    // "ci" doesn't refer to a release action type
    let incorrect_message_ci_not_match = "ci: some change".to_string();
    let incorrect_commit1 = Commit::new(incorrect_message_ci_not_match, "somehash".to_string());
    // No semi-column after the type
    let incorrect_message_no_semicolumn = "feat introduced new function".to_string();
    let incorrect_commit2 = Commit::new(incorrect_message_no_semicolumn, "somehash".to_string());

    // Execution step
    let analyzer = CommitAnalyzerPlugin::default();

    // Asserts the results of the function match the correct ReleaseAction.
    assert_eq!(
        analyzer.execute(&correct_commit1, &config.release_rules)?,
        ReleaseAction::Major
    );
    assert_eq!(
        analyzer.execute(&correct_commit2, &config.release_rules)?,
        ReleaseAction::Patch
    );

    // Asserts the results of the function are incorrects.
    assert!(analyzer.execute(&incorrect_commit1, &config.release_rules).is_err());
    assert!(analyzer.execute(&incorrect_commit2, &config.release_rules).is_err());

    Ok(())
}

// Tests the function `analyze`.
//
// The `analyze` function analyzes multiple messages and return the highest
// [ReleaseAction] found from them.
// If a ReleaseAction is not found, a `None` is returned.
#[test]
fn test_can_run() {
    // Unit test preparation
    let context = Context::default();

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
    let mut correct_messages_major_release = vec![
        Commit::new("break: add a function".to_string(), "somehash".to_string()),
        Commit::new("refac: some ref".to_string(), "somehash".to_string()),
        Commit::new("ci: some change".to_string(), "somehash".to_string()),
        Commit::new("feat: a cool feature".to_string(), "somehash".to_string()),
    ];

    let mut correct_messages_patch_release = vec![
        Commit::new("refac: documentation".to_string(), "somehash".to_string()),
        Commit::new("refac: some ref".to_string(), "somehash".to_string()),
        Commit::new("ci: some change".to_string(), "somehash".to_string()),
    ];

    let mut correct_no_release: Vec<Commit> = vec![];

    // Execution step
    let analyzer = CommitAnalyzerPlugin::default();

    // Asserts the results of the function matches the correct ReleaseAction
    assert_eq!(
        analyzer
            .run(&context, &mut correct_messages_major_release, &config.release_rules)
            .unwrap(),
        ReleaseAction::Major
    );

    assert_eq!(
        analyzer
            .run(&context, &mut correct_messages_patch_release, &config.release_rules)
            .unwrap(),
        ReleaseAction::Patch
    );

    assert!(analyzer
        .run(&context, &mut correct_no_release, &config.release_rules)
        .is_none());

    // Asserts the commit's release action type changed correctly
    assert_eq!(
        correct_messages_major_release[0].release_action,
        Some(ReleaseAction::Major)
    );

    assert_eq!(
        correct_messages_major_release[1].release_action,
        Some(ReleaseAction::Patch)
    );

    assert_eq!(correct_messages_major_release[2].release_action, None);

    assert_eq!(
        correct_messages_major_release[3].release_action,
        Some(ReleaseAction::Minor)
    );
}

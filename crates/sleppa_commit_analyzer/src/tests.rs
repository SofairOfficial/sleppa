//! Unit tests
//!
//! This testing module implements the unit tests for testing the commit analyzer routines.

use super::*;

#[test]
/// Tests the function `execute`.
fn test_can_execute() {
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
    let msg0 = "break: add a function";
    let msg1 = "refac: some ref";

    // Creates incorrect messages for the grammar defined above
    // "ci" doesn't refer to a release action type
    let msg2 = "ci: some change";
    // No semi-column after the type
    let msg3 = "feat introduced new function";

    // Execution step
    let ca = CommitAnalyzer::default();

    // Asserts the results of the function match the correct ReleaseAction
    assert_eq!(ca.execute(msg0, &config.release_rules).unwrap(), ReleaseAction::Major);
    assert_eq!(ca.execute(msg1, &config.release_rules).unwrap(), ReleaseAction::Patch);

    // Asserts the results of the function are incorrect.
    assert!(ca.execute(msg2, &config.release_rules).is_err());
    assert!(ca.execute(msg3, &config.release_rules).is_err());
}

#[test]
/// Tests the function `action_to_release`.
fn test_can_action_to_release() {
    // Unit test preparation
    // Builds a correct [Configuration] structure testing purpose.
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

    // Creates [MessageToAnalyze] structures
    let messages1 = vec![
        "break: add a function".to_string(),
        "refac: some ref".to_string(),
        "ci: some change".to_string(),
        "feat: a cool feature".to_string(),
    ];

    let messages2 = vec![
        "refac: documentation".to_string(),
        "refac: some ref".to_string(),
        "ci: some change".to_string(),
    ];

    let messages3: Vec<String> = vec![];

    let mes1 = MessagesToAnalyze { messages: messages1 };
    let mes2 = MessagesToAnalyze { messages: messages2 };
    let mes3 = MessagesToAnalyze { messages: messages3 };

    // Execution step
    let ca = CommitAnalyzer::default();

    // Asserts the results of the function match the correct ReleaseAction
    assert_eq!(
        ca.action_to_release(mes1, &config.release_rules).unwrap(),
        ReleaseAction::Major
    );
    assert_eq!(
        ca.action_to_release(mes2, &config.release_rules).unwrap(),
        ReleaseAction::Patch
    );
    assert!(ca.action_to_release(mes3, &config.release_rules).is_none());
}

//! Unit tests
//!
//! This testing module implements the unit tests for testing the commit analyzer routines.

use crate::{errors::TestResult, *};
use sleppa_configuration::Configuration;
use std::collections::HashMap;

// Test to retrieve the correct ReleaseAction from message.
//
// The `execute` function must return a [ReleaseAction] if the message matches the defined regex.
// A [CommitAnalyzerError] is returned if no matches.
#[test]
fn test_can_execute() -> TestResult<()> {
    // Unit test preparation
    // Builds a default [Configuration] structure for testing purpose.
    let config: CommitAnalyzerConfiguration = CommitAnalyzerConfiguration::default();

    // Creates correct messages for the grammar defined above
    let correct_message_major_release_action = "break: add a function".to_string();
    let correct_commit1 = Commit::new(correct_message_major_release_action, "somehash".to_string());

    let correct_message_patch_release_action = "style: some ref".to_string();
    let correct_commit2 = Commit::new(correct_message_patch_release_action, "somehash".to_string());

    // Creates incorrect messages for the grammar defined above
    // "broke" doesn't refer to a release action type
    let incorrect_message_ci_not_match = "broke: some change".to_string();
    let incorrect_commit1 = Commit::new(incorrect_message_ci_not_match, "somehash".to_string());
    // No semi-column after the type
    let incorrect_message_no_semicolumn = "feat introduced new function".to_string();
    let incorrect_commit2 = Commit::new(incorrect_message_no_semicolumn, "somehash".to_string());

    // Execution step
    let analyzer = CommitAnalyzerPlugin {
        configuration: config.clone(),
    };

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
    let mut context = Context {
        configurations: HashMap::new(), //HashMap<String, Configuration>
    };

    let config = Configuration {
        map: HashMap::new(), //HashMap<String, Value>
    };

    context.configurations.insert(CONFIGURATION_KEY.to_string(), config);

    // Builds a default [Configuration] structure for testing purpose.
    let ca_config = CommitAnalyzerConfiguration::default();

    // Creates arrays of strings
    let correct_messages_major_release = vec![
        Commit::new("break: add a function".to_string(), "somehash".to_string()),
        Commit::new("style: some ref".to_string(), "somehash".to_string()),
        Commit::new("broke: some change".to_string(), "somehash".to_string()),
        Commit::new("feat: a cool feature".to_string(), "somehash".to_string()),
    ];

    let correct_messages_patch_release = vec![
        Commit::new("style: documentation".to_string(), "somehash".to_string()),
        Commit::new("style: some ref".to_string(), "somehash".to_string()),
        Commit::new("broke: some change".to_string(), "somehash".to_string()),
    ];

    let correct_no_release: Vec<Commit> = vec![];

    // Execution step
    let analyzer = CommitAnalyzerPlugin {
        configuration: ca_config,
    };

    context
        .configurations
        .get_mut(&CONFIGURATION_KEY.to_string())
        .unwrap()
        .map
        .insert(
            CONFIGURATION_COMMITS.to_string(),
            Value::Commits(correct_messages_major_release),
        );

    // Asserts the results of the function matches the correct ReleaseAction
    assert_eq!(analyzer.run(&mut context).unwrap(), Some(ReleaseAction::Major));

    // Asserts the commit's release action type changed correctly
    assert_eq!(
        context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_COMMITS.to_string()]
            .as_commits()
            .unwrap()[0]
            .release_action,
        Some(ReleaseAction::Major)
    );

    assert_eq!(
        context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_COMMITS.to_string()]
            .as_commits()
            .unwrap()[1]
            .release_action,
        Some(ReleaseAction::Patch)
    );

    assert_eq!(
        context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_COMMITS.to_string()]
            .as_commits()
            .unwrap()[2]
            .release_action,
        None
    );

    assert_eq!(
        context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_COMMITS.to_string()]
            .as_commits()
            .unwrap()[3]
            .release_action,
        Some(ReleaseAction::Minor)
    );

    context
        .configurations
        .get_mut(&CONFIGURATION_KEY.to_string())
        .unwrap()
        .map
        .insert(
            CONFIGURATION_COMMITS.to_string(),
            Value::Commits(correct_messages_patch_release),
        );
    assert_eq!(analyzer.run(&mut context).unwrap(), Some(ReleaseAction::Patch));

    context
        .configurations
        .get_mut(&CONFIGURATION_KEY.to_string())
        .unwrap()
        .map
        .insert(CONFIGURATION_COMMITS.to_string(), Value::Commits(correct_no_release));
    assert!(analyzer.run(&mut context).unwrap().is_none());
}

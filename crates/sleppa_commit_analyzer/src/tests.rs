//! Unit tests
//!
//! This testing module implements the unit tests for testing the commit analyzer routines.

use crate::{errors::TestResult, *};
use sleppa_primitives::repositories::github::GithubRepository;
use std::collections::HashMap;

// Retrieves the correct ReleaseAction from message.
//
// The `execute` function must return a [ReleaseAction] if the message matches the defined regex.
// A [CommitAnalyzerError] is returned if no matches.
#[test]
fn test_can_execute() -> TestResult<()> {
    // Unit test preparation
    // Builds a default configuration structure for testing purpose.
    let config = CommitAnalyzerConfiguration::default();

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

// Retrieves a correct `ReleaseAction` from given commits
#[test]
fn test_can_run() {
    // Unit test preparation
    let repo = GithubRepository {
        owner: "owner".to_string(),
        repo: "repo".to_string(),
    };
    let mut context = Context {
        map: HashMap::new(),
        repository: repo,
    };

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
    let analyzer = CommitAnalyzerPlugin::new();

    context.map.insert(
        CONTEXT_COMMITS.to_string(),
        Value::Commits(correct_messages_major_release),
    );

    // Asserts the results of the function matches the correct ReleaseAction
    assert_eq!(analyzer.run(&mut context).unwrap(), Some(ReleaseAction::Major));

    // Asserts the commit's release action type changed correctly
    assert_eq!(
        context.map[CONTEXT_COMMITS].as_commits().unwrap()[0].release_action,
        Some(ReleaseAction::Major)
    );

    assert_eq!(
        context.map[CONTEXT_COMMITS].as_commits().unwrap()[1].release_action,
        Some(ReleaseAction::Patch)
    );

    assert_eq!(
        context.map[CONTEXT_COMMITS].as_commits().unwrap()[2].release_action,
        None
    );

    assert_eq!(
        context.map[CONTEXT_COMMITS].as_commits().unwrap()[3].release_action,
        Some(ReleaseAction::Minor)
    );

    // Asserts the results of the function matches the correct ReleaseAction
    context.map.insert(
        CONTEXT_COMMITS.to_string(),
        Value::Commits(correct_messages_patch_release),
    );
    assert_eq!(analyzer.run(&mut context).unwrap(), Some(ReleaseAction::Patch));

    // Asserts the results of the function matches the correct ReleaseAction
    context
        .map
        .insert(CONTEXT_COMMITS.to_string(), Value::Commits(correct_no_release));
    assert!(analyzer.run(&mut context).unwrap().is_none());
}

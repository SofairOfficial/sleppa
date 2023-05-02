//! Unit tests
//!
//! This testing module implements the unit tests for testing the commit analyzer routines.

use super::repositories::{github::GithubRepository, Repository};
use super::*;

/// Tests the function `execute`.
///
/// The `execute` function must return a [ReleaseAction] if the message matches the defined regex.
/// A [CommitAnalyzerError] is returned if no matches.
#[test]
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
    let correct_message_major_release_action = "break: add a function";
    let correct_message_patch_release_action = "refac: some ref";

    // Creates incorrect messages for the grammar defined above
    // "ci" doesn't refer to a release action type
    let incorrect_message_ci_not_match = "ci: some change";
    // No semi-column after the type
    let incorrect_message_no_semicolumn = "feat introduced new function";

    // Execution step
    let analyzer = CommitAnalyzer::default();

    // Asserts the results of the function match the correct ReleaseAction.
    assert_eq!(
        analyzer
            .execute(correct_message_major_release_action, &config.release_rules)
            .unwrap(),
        ReleaseAction::Major
    );
    assert_eq!(
        analyzer
            .execute(correct_message_patch_release_action, &config.release_rules)
            .unwrap(),
        ReleaseAction::Patch
    );

    // Asserts the results of the function are incorrects.
    assert!(analyzer
        .execute(incorrect_message_ci_not_match, &config.release_rules)
        .is_err());
    assert!(analyzer
        .execute(incorrect_message_no_semicolumn, &config.release_rules)
        .is_err());
}

/// Tests the function `analyze`.
///
/// The `analyze` function analyzes multiple messages and return the highest
/// [ReleaseAction] found from them.
/// If a ReleaseAction is not found, a `None` is returned.
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
    let analyzer = CommitAnalyzer::default();

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

/// Tests the function `get_pull_request_number_from_its_name`.
///
/// This function analyzes the name of a pull request and retrieve its number as u64.
/// If none is found a [RepositoriesError] is returned.
#[test]
fn test_can_get_pull_request_number_from_its_name() {
    // Unit test preparation
    // Builds a correct pull request's name.
    let correct_pr_name = "Issue to solve (#2)";

    // Builds  incorrect pull request's names.
    // An incorrect space
    let incorrect_space = "Issue to solve (# 3)";
    // No parenthesis
    let incorrect_no_parenthesis = "Issue to solve #3";
    // No hashtag
    let incorrect_no_hashtag = "Issue to solve (5)";

    // Asserts the pull request's number is retrieved
    assert_eq!(
        GithubRepository::get_pull_request_number_from_its_name(correct_pr_name).unwrap(),
        2u64
    );
    // Asserts an error occurs
    assert!(GithubRepository::get_pull_request_number_from_its_name(incorrect_space).is_err());
    assert!(GithubRepository::get_pull_request_number_from_its_name(incorrect_no_parenthesis).is_err());
    assert!(GithubRepository::get_pull_request_number_from_its_name(incorrect_no_hashtag).is_err());
}

/// Tests the function `get_last_tag`.
///
/// This function retrieves the last tag of a repository. As it works with [octocrab], the http request
/// is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
/// repository has been created.
/// The tag of this repo is "v1.0.0" and its associated hash : "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a"
#[tokio::test]
async fn test_can_get_last_tag() -> TestResult<()> {
    // Unit test preparation
    // Providing the credentials for the test repository.
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };

    // Execution step
    let response = githubrepository.get_last_tag().await?;

    // Asserts the name of the tag and its hash are ok.
    assert_eq!(response.tag_id, "v1.0.0");
    assert_eq!(response.tag_hash, "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a");
    Ok(())
}

/// Tests the function `get_pull_request`.
///
/// This function retrieves the pull request since a tag. As it works with [octocrab], the http request
/// is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
/// repository has been created.
/// The tag of this repo is "v1.0.0" and its associated hash : "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a"
/// 3 pull requests have been done on the repository :
///  - Issue-to-solve-3
///  - Issue-to-solve-2 (#2)
///  - Issue-to-solve-1 (#1)
///
/// As the `Issue-to-solve-1 (#1)` is linked to the last tag, it will be ignored.
#[tokio::test]
async fn test_can_get_pull_request() -> TestResult<()> {
    // Unit test preparation
    // Providing the credentials for the test repository.
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };
    let tag_sha = "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a";

    // Execution step
    let response = githubrepository.get_pull_request(tag_sha).await?;

    // Asserts the name of retrived pull request are corrects.
    assert!(response.len() == 2);
    assert_eq!(response[0], "Issue-to-solve-3");
    assert_eq!(response[1], "Issue-to-solve-2 (#2)");
    Ok(())
}

/// Tests the function `get_inner_commits_from_pull_request`.
///
/// This function retrieves the pull request's inner commits from a pull request number. As it works with [octocrab],
/// the http request is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
/// repository has been created.
/// The last valid pull request number comes from the name "Issue-to-solve-2 (#2)", hence the pull request's number to
/// analyze is `2`.
#[tokio::test]
async fn test_can_get_inner_commits_from_pull_request() -> TestResult<()> {
    // Unit test preparation
    // Providing the credentials for the test repository.
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };
    let pull_request_number = 2u64;

    // Execution step
    let response = githubrepository
        .get_inner_commits_from_pull_request(pull_request_number)
        .await?;

    // Asserts the name of pull request's inner commits are corrects.
    assert_eq!(response[0].commit.message, "patch:some patch");
    assert_eq!(response[1].commit.message, "feat(script): add a script");
    assert_eq!(response[2].commit.message, "feat: add a feature");

    Ok(())
}

/// Tests the function `get_inner_commits_messages`.
///
/// This function retrieves the pull request's inner commits message from a repository. As it works with [octocrab],
/// the http request is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
/// repository has been created.
/// The function retrieves the last tag (`v1.0.0`), get the pull requests since this tag, analyzes their name, extracts the
/// pull request number and retrieves all the inner commit's messages.
#[tokio::test]
async fn test_can_get_inner_commits_messages() -> TestResult<()> {
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };

    // Execution step
    let response = githubrepository.get_inner_commits_messages().await?;

    // Asserts the name of retrived pull request are corrects.
    assert_eq!(response[0], "patch:some patch");
    assert_eq!(response[1], "feat(script): add a script");
    assert_eq!(response[2], "feat: add a feature");

    Ok(())
}

/// Unit test result type
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

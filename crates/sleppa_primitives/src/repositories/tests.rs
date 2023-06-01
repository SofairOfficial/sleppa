//! Unit tests
//!
//! This testing module implements the unit tests for testing the repositories module routines.

use crate::repositories::{errors::TestResult, github::GithubRepository, *};
use std::env::set_var;

// Tests to retrieve a pull request number's from it's name.
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

// Tests to retrieve the last tag of a GitHub repository.
//
// This function retrieves the last tag of a repository. As it works with [octocrab], the http request
// is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
// repository has been created.
// The tag of this repo is "v1.0.0" and its associated hash : "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a"
#[tokio::test]
#[ignore = "test done in test_can_get_inner_commits"]
async fn test_can_get_last_tag() -> TestResult<()> {
    // Unit test preparation
    // Providing the credentials for the test repository.
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };

    // Execution step
    let response = githubrepository.get_tag().await?;

    // Asserts the name of the tag and its hash are ok.
    assert_eq!(response.identifier, "v1.0.0");
    assert_eq!(response.hash, "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a");
    Ok(())
}

// Tests to retrieve the pull request since a given tag for a GitHub reposiroty.
//
// This function retrieves the pull request since a tag. As it works with [octocrab], the http request
// is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
// repository has been created.
// The tag of this repo is "v1.0.0" and its associated hash : "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a"
// 3 pull requests have been done on the repository :
//  - Issue-to-solve-3
//  - Issue-to-solve-2 (#2)
//  - Issue-to-solve-1 (#1)
//
// As the `Issue-to-solve-1 (#1)` is linked to the last tag, it will be ignored.
#[tokio::test]
#[ignore = "test done in test_can_get_inner_commits"]
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

// Tests the function `get_inner_commits_from_pull_request`.
//
// This function retrieves the pull request's inner commits from a pull request number. As it works with [octocrab],
// the http request is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
// repository has been created.
// The last valid pull request number comes from the name "Issue-to-solve-2 (#2)", hence the pull request's number to
// analyze is `2`.
#[tokio::test]
#[ignore = "test done in test_can_get_inner_commits"]
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

// Tests to retrieve the inner commits messages of a squashed-and-merged pull request
// of a GitHub repository
//
// This function retrieves the pull request's inner commits message from a repository. As it works with [octocrab],
// the http request is automatically sent to GitHub. Therefore a [semantic release testbed](https://github.com/SofairOfficial/semantic-release-squash-and-merge-testbed)
// repository has been created.
#[tokio::test]
#[ignore = "time consuming test"]
async fn test_can_get_inner_commits() -> TestResult<()> {
    set_var("GITHUB_TOKEN", "token");
    let githubrepository = GithubRepository {
        repo: "semantic-release-squash-and-merge-testbed".to_string(),
        owner: "SofairOfficial".to_string(),
    };

    // Execution step
    let response = githubrepository.get_inner_commits().await?;

    // Asserts the name of retrieved pull request are corrects.
    assert_eq!(response[0].message, "patch:some patch");
    assert_eq!(response[1].message, "feat(script): add a script");
    assert_eq!(response[2].message, "feat: add a feature");

    assert_eq!(response[0].hash, "76c8e718b043764de6201aabfe05c0e9ee0cf3a2");
    assert_eq!(response[1].hash, "912dcb7b5ab87570ca3df3b7c465ac8a3505cc04");
    assert_eq!(response[2].hash, "f474fce23d7fd18395681949e5c16194b0400297");

    Ok(())
}

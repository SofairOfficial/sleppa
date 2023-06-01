//! Wrapper around GitHub API
//!
//! [Octocrab](https://crates.io/crates/octocrab) crate is used to retrieve pull requests, their inner commits and the tag since the last release
//! from a GitHub repository.
//!
//! The semantic API of octocrab is used when possible. However the HTTP API is used to retrieve the pull request's
//! inner commit as this request is not implemented by the semantic API.
//!
//! Octocrab semantic API are built as `Builder` structures using methods with multiple optionnal parameters e.g. :
//! `let mut page = octocrab::instance().issues("octocrab", "repo").list().creator("octocrab").per_page(50).send().await?;`
//!
//! This builder returns a listing represented as [octocrab::Page] structure.
//!
//! The pull requests are [RepoCommit] structure. It contains a field \[RepoCommit::commit\] where the message is
//! stored inside a [octocrab::models::repos::RepoCommitPage] structure along with other fields.
//!
//! The inner commit's are [RepoCommit] structure as well.
//!
//! To disambiguate, in octocrab a [octocrab::models::pulls::PullRequest] is a pull request item which state can be
//! opened or closed.
//! Once the pull request has been merged to a branch, it is available as a [RepoCommit] with its own properties like
//! message and hash.

use crate::Commit;
use async_trait::async_trait;
use octocrab::models::repos::RepoCommit;
use regex::Regex;

use super::{
    errors::{RepositoryError, RepositoryResult},
    GitRepository, RepositoryTag,
};

/// A minimal GitHub repository structure
///
/// A GitHub repository comes with at least two parameters, namely:
/// - an owner
/// - a name
///
/// The path is then like `/repos/{owner}/{name}/` for the GitHub's API
#[derive(Clone, Default, Debug)]
pub struct GithubRepository {
    /// Represents the owner
    pub owner: String,
    /// Represents the name of the repository
    pub repo: String,
}

#[async_trait]
impl GitRepository for GithubRepository {
    /// Get the reposiroty's last tag and its sha
    ///
    /// If the repository has no tag yet, an empty one is created.
    /// Else the repository's tag is used to create a new [RepositoryTag].
    ///
    /// The octocrab semantic API returns a [octocrab::Page] of [octocrab::Tag].
    async fn get_tag(&self) -> RepositoryResult<RepositoryTag> {
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;
        // Get all the tag of a repository.
        let page_tags = octocrab.repos(&self.owner, &self.repo).list_tags().send().await?;

        if page_tags.items.is_empty() {
            // Creates an empty [RepositoryTag] if no tag is found.
            let last_tag = RepositoryTag {
                identifier: "".to_string(),
                hash: "".to_string(),
            };
            Ok(last_tag)
        } else {
            // Creates a [RepositoryTag] with the tag found.
            let last_tag = &page_tags.items[0];
            Ok(RepositoryTag {
                identifier: last_tag.name.to_string(),
                hash: last_tag.commit.sha.to_string(),
            })
        }
    }

    /// Get inner commit messages since the last tag
    ///
    /// From a repository's name and owner, all the inner commits since the last tag are retrieved.
    /// If no tag is found, all the [RepoCommit] are analyzed.
    /// If the name of the pull request is malformed, it is then ignored.
    async fn get_inner_commits(&self) -> RepositoryResult<Vec<Commit>> {
        let mut commits: Vec<Commit> = vec![];

        // Get the repository's tag.
        let tag = self.get_tag().await?;

        // Get the repository's pull request from the tag.
        let repo_commits = self.get_pull_request(&tag.hash).await?;

        // Extracts the pull request's number from its name.
        // If the pull request's name is malformed, the procces ignores it.
        for name in repo_commits {
            let pr_number = match GithubRepository::get_pull_request_number_from_its_name(&name) {
                Ok(pr_number) => pr_number, // Get the pull request's number
                Err(_err) => continue,      // Ignore malformed pull request's name
            };

            // Get the inner commits from the pull request's number found previously
            let inner_commits = self.get_inner_commits_from_pull_request(pr_number).await?;

            // Pushes inner commit messages to the result array
            for commit in inner_commits {
                commits.push(Commit::new(commit.commit.message.to_string(), commit.sha.to_string()));
            }
        }
        Ok(commits)
    }

    async fn push_release(&self, tag: RepositoryTag) -> RepositoryResult<()> {
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        // Build an octocrab instance with the provided credentials.
        let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;

        // Publishes the release for the given tag.
        octocrab
            .repos(self.owner.as_str(), self.repo.as_str())
            .releases()
            .create(&tag.identifier)
            .target_commitish("main")
            .send()
            .await?;
        Ok(())
    }

    fn get_url(&self) -> String {
        format!("https://github.com/{}/{}", self.owner, self.repo)
    }
}

impl GithubRepository {
    /// Get the pull request's name
    ///
    /// In a squash-and-merge strategy, the merged commits are pull-request. Therefore their name
    /// must be well formed e.g. "Issue to solve (#2)" in order to retrieve their number.
    ///
    /// The octocrab Semantic API returns a [octocrab::Page] of [RepoCommit].
    pub async fn get_pull_request(&self, tag_sha: &str) -> RepositoryResult<Vec<String>> {
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;

        let repo_commits = octocrab.repos(&self.owner, &self.repo).list_commits().send().await?;

        let mut pull_request_messages: Vec<String> = vec![];

        if tag_sha.is_empty() {
            // Retrieves all the repository commit of a repository if there is no tag
            for item in repo_commits.items {
                pull_request_messages.push(item.commit.message.to_string())
            }
        } else {
            // If a tag is found, only the repository commits until this tag are retrieved
            for item in repo_commits.items {
                if item.sha != tag_sha {
                    pull_request_messages.push(item.commit.message.to_string())
                } else {
                    break;
                }
            }
        }

        Ok(pull_request_messages)
    }

    /// Get the pull request's number from its name
    ///
    /// In a squash-and-merge strategy, the merged pull request must have a name well formed like `Issue to solve (#6)`
    /// where `6` indicates the pull request's number. That number is retrieved by this function using a regex format.
    pub fn get_pull_request_number_from_its_name(pull_request_name: &str) -> RepositoryResult<u64> {
        // Creates the regex expression for the pull request's name grammar e.g. `Issue to solve (#5)`.
        let regex = Regex::new(r"\([a-z]*\#(?P<number>[0-9]+)\)$")?;

        // Verifies if the grammar matches the pull request's name
        let captured = match regex.captures(pull_request_name) {
            Some(captured) => captured,
            None => return Err(RepositoryError::InvalidMessage("Fails to match regex".to_string())),
        };

        // Get the captured group `number` to get the pull request's number
        let pr_number = match captured.name("number") {
            Some(number) => number,
            None => {
                return Err(RepositoryError::InvalidMessage(
                    "Fails to captured the group".to_string(),
                ))
            }
        };

        // Parses the string slice to an u64
        match pr_number.as_str().parse::<u64>() {
            Ok(number) => Ok(number),
            Err(err) => Err(RepositoryError::ParsingError(err)),
        }
    }

    /// Get pull request's inner commits
    ///
    /// From the pull request's number, its inner commits are retrieved thanks to [octocrab] HTTP API.
    /// The inner commit of a pull request are [RepoCommit] in octocrab.
    pub async fn get_inner_commits_from_pull_request(&self, pr_number: u64) -> RepositoryResult<Vec<RepoCommit>> {
        let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;

        // Format the route to the repository
        let repo_address = format! {"/repos/{}/{}/pulls/{}/commits", &self.owner, &self.repo, pr_number};

        // Retrieve the inner commits with the octocrab HTTP API
        let commits = octocrab.get(repo_address, None::<&()>).await?;
        Ok(commits)
    }
}

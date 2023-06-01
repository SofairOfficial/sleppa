//! Sleppa version incrementing package
//!
//! This crate aims at incrementing a version number, namely a [Tag] (formed as `v3.2.1`),
//! according to a [ReleaseAction].
//! A [Tag] is composed of 3 digits, one for each release action type, namely, major or minor or patch.
//! The type of release action affects the tag differently:
//!  - major: adds 1 to the first digit and set 0 to others, e.g. from `3.2.1` -> `4.0.0`,
//!  - minor: adds 1 to the second and set 0 to the third, e.g. from `3.2.1` -> `3.3.0`,
//!  - patch: adds 1 to the third, e.g. from `3.2.1` -> `3.2.2`.
//!
//! Datas used to create the new version are retrieved from a [Context] structure.
//! This [Context] should contain [CONTEXT_LAST_TAG] to access the last tag of the repository, [CONTEXT_USER] to access
//! the user to commit the [Tag] and a [CONTEXT_RELEASE_ACTION] to access the release action type found.

mod errors;

use errors::{VersionerError, VersionerResult};
use regex::Regex;
use sleppa_primitives::{
    repositories::{GitRepository, RepositoryUser},
    Context, ReleaseAction,
};

use std::process::Command;

pub struct VersionerPlugin;

/// Definition of a Tag
///
/// A tag is defined like `v3.2.1` where `v{major}.{minor}.{patch}`
#[derive(Debug, PartialEq)]
pub struct Tag {
    /// Major number defining a tag
    major: u64,
    /// Minor number defining a tag
    minor: u64,
    /// Patch number defining a tag
    patch: u64,
}

impl VersionerPlugin {
    /// Calculates the new Tag for a given release action
    ///
    /// This function takes an existing [Tag] and calculates the new tag for a given [ReleaseAction].
    pub fn run<R: GitRepository>(&self, context: &Context<R>) -> VersionerResult<Tag> {
        let last_tag = match context.load_last_tag() {
            Some(value) => value,
            None => return Err(VersionerError::InvalidContext("missing last tag".to_string())),
        };

        let user = match context.load_user() {
            Some(value) => value,
            None => return Err(VersionerError::InvalidContext("missing user".to_string())),
        };

        let release_action = match context.load_release_action() {
            Some(value) => value,
            None => return Err(VersionerError::InvalidContext("missing release action".to_string())),
        };

        let tag = Tag::try_from(last_tag.identifier.as_str())?;

        let new_tag = tag.increment(&release_action);
        self.commit_tag(&user, &new_tag)?;

        Ok(new_tag)
    }

    /// Commits the new changelog file and the new tag
    ///
    /// This function commits the file to the repository with the provided path.
    /// The commit message is like `Release v3.2.1`.
    fn commit_tag(&self, user: &RepositoryUser, tag: &Tag) -> VersionerResult<()> {
        let commit_user = format!(r#"git config user.name "{}""#, user.name);
        let commit_user_email = format!(r#"git config user.email "{}""#, user.email);
        let commit_tag = format!(r#"git tag {}"#, tag);

        Command::new("sh").arg("-c").arg(commit_user).status().expect("failed");

        Command::new("sh").arg("-c").arg(commit_user_email).status()?;

        Command::new("sh").args(["-c", commit_tag.as_str()]).status()?;

        Command::new("sh").args(["-c", "git push --tags"]).status()?;

        Ok(())
    }
}

impl TryFrom<&str> for Tag {
    type Error = VersionerError;

    /// Tries to convert from a tag as string to a tag as structure
    ///
    /// This function tries to convert a given tag defined as string to a [Tag] defined as structure.
    fn try_from(tag: &str) -> VersionerResult<Tag> {
        // Creates the regex grammar to match a tag formed like `v3.2.1`.
        // This regex grammar defines named captured groups for major, minor and patch number.
        let regex = Regex::new("^v{1}(?P<major>[0-9]+).(?P<minor>[0-9]+).(?P<patch>[0-9]+)$")?;
        let captured = match regex.captures(tag) {
            Some(captured) => captured,
            None => return Err(VersionerError::ErrorNoMatch("regex".to_string())),
        };

        // Evaluates if the captured groups are correct
        let major = match captured.name("major") {
            Some(major) => major.as_str(),
            None => return Err(VersionerError::ErrorNoMatch("major number".to_string())),
        };

        let minor = match captured.name("minor") {
            Some(minor) => minor.as_str(),
            None => return Err(VersionerError::ErrorNoMatch("minor number".to_string())),
        };

        let patch = match captured.name("patch") {
            Some(patch) => patch.as_str(),
            None => return Err(VersionerError::ErrorNoMatch("patch number".to_string())),
        };

        // Parses the captured groups from char to u64
        let tag = Tag {
            major: major.parse::<u64>()?,
            minor: minor.parse::<u64>()?,
            patch: patch.parse::<u64>()?,
        };

        Ok(tag)
    }
}

impl TryInto<String> for Tag {
    type Error = VersionerError;

    /// Implements the parsing from [Tag] to [String]
    ///
    /// The `try_into` method cannot be explicit from the previous `try_from`,then it should be implemented.
    fn try_into(self) -> VersionerResult<String> {
        Ok(format!("v{}.{}.{}", self.major, self.minor, self.patch))
    }
}

impl std::fmt::Display for Tag {
    /// Prints the correct format for Tag e.g. "v3.2.1".
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Tag {
    /// Increments the tag according to the release action
    ///
    /// A [Tag] is composed of 3 digits, e.g. `v3.2.1`. According to a [ReleaseAction], these digits
    /// are incremented. It adds:
    ///  - 1 to the first digit and set 0 to others for major, e.g. from `3.2.1` -> `4.0.0`,
    ///  - 1 to the second and set 0 to the third for minor, e.g. from `3.2.1` -> `3.3.0`,
    ///  - 1 to the third for patch, e.g. from `3.2.1` -> `3.2.2`.
    fn increment(&self, release_action: &ReleaseAction) -> Self {
        let mut tag = Tag {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
        };
        match release_action {
            ReleaseAction::Major => {
                tag.major += 1;
                tag.minor = 0;
                tag.patch = 0;
                tag
            }
            ReleaseAction::Minor => {
                tag.minor += 1;
                tag.patch = 0;
                tag
            }
            ReleaseAction::Patch => {
                tag.patch += 1;
                tag
            }
        }
    }
}

#[cfg(test)]
mod tests;

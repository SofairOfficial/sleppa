//! Sleppa changelog file generator
//!
//! This package aims at generating the changelog file for a given semantic release. For doing so,
//! it takes as inputs the Git tags from the latest and newest releases, as well as the list of
//! conventionnal commits, and generate an output file.
//!
//! The commits are loaded and according to their message, different sections in the file
//! will be written.
//! These sections represent the sorted type of commit.
//!
//! The changelog file looks like :
//!
//!```toml
//! ### [v4.0.0](https://github.com/user/repo/compare/v3.2.1..v4.0.0) (2023-05-05)
//!
//! * **Major changes**
//!  * break: new breaking ([1ebdf43e](https://github.com/user/repo/commit/1ebdf43e8950d8f9dace2e554be5d387267575ef))
//! * **Minor changes**
//!  * feat: new feature ([172cd158](https://github.com/user/repo/commit/172cd1589d0a29b56cd8261a888911201305b04d))
//!  * refac: refac a feature ([987cd158](https://github.com/user/repo/commit/8987cd1589d0a29b56cd8261a888911201305b04d))
//! * **Patch changes**
//!  * style: style change ([cd2fe770](https://github.com/user/repo/commit/cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))
//!```
//!
//! While the file is written, it has to be automatically commited to the reposiroty with a message : `Release v4.0.0`
//! where `v4.0.0` is the new tag.

pub mod constants;
mod errors;

use constants::CHANGELOG_DEFAULT_PATH;
use errors::{ChangelogError, ChangelogResult};
use sleppa_configuration::constants::{
    CONFIGURATION_COMMITS, CONFIGURATION_KEY, CONFIGURATION_LAST_TAG, CONFIGURATION_NEW_TAG, CONFIGURATION_USER,
};
use sleppa_configuration::Context;
use sleppa_primitives::repositories::RepositoryUser;
use sleppa_primitives::{Commit, ReleaseAction};
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use time::{format_description, OffsetDateTime};

/// Defines the Changelog and its fields.
///
/// Changelog structure contains mandatory elements to create the file, namely, the map between commit type and
/// commit messages, the last tag, the new tag and the URL of the repository.
/// The URL is used to write hlink in the changelog file, therefore using a String here is sufficient.
#[derive(Default)]
pub struct ChangelogPlugin {
    /// Sections are represented by the three [ReleaseAction] type (the keys) associated with their [Commit]s (the value).
    /// As the order of the key is important, a [BTreeMap] is needed here.
    pub sections: BTreeMap<ReleaseAction, Vec<Commit>>,
    /// The reposiroty's previous tag
    pub last_tag: String,
    /// The repository's new tag
    pub new_tag: String,
    /// The repository's URL like `https://github.com/USER/REPO`
    pub repo_url: String,
}

impl ChangelogPlugin {
    /// Implementation of the `new` method : `ChangelogPlugin::new()`.
    pub fn new() -> Self {
        ChangelogPlugin::default()
    }

    /// Executes the main function of the changelog generator plugin
    ///
    /// This function builds the [ChangelogPlugin] from a vector of [Commit]s and writes the file to a
    /// provided path.
    /// The file is written using the commits messages as source of information. The changelog groups the
    /// commits using their [ReleaseAction] type.
    ///
    /// The `repo_url` argument is used to write the file html link. Whereas the `changelog_path` is the path where to
    /// write the CHANGELOG.md file.
    pub fn run(&mut self, context: &Context, changelog_path: &Path, repo_url: &str) -> ChangelogResult<()> {
        let commits = match context.configurations[&CONFIGURATION_KEY.to_string()].map
            [&CONFIGURATION_COMMITS.to_string()]
            .as_commits()
        {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No commits found.".to_string())),
        };

        let last_tag = match context.configurations[&CONFIGURATION_KEY.to_string()].map
            [&CONFIGURATION_LAST_TAG.to_string()]
            .as_tag()
        {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No last tag found.".to_string())),
        };

        let new_tag = match context.configurations[&CONFIGURATION_KEY.to_string()].map
            [&CONFIGURATION_NEW_TAG.to_string()]
            .as_tag()
        {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No new tag found.".to_string())),
        };

        let user = match context.configurations[&CONFIGURATION_KEY.to_string()].map[&CONFIGURATION_USER.to_string()]
            .as_user()
        {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("missing user".to_string())),
        };

        // Builds the [ChangelogPlugin] from commits, last tag and new tag
        self.with_commits(
            commits,
            last_tag.identifier.as_str(),
            new_tag.identifier.as_str(),
            repo_url,
        );

        // Creates the changelog file at the given path
        self.serialize(changelog_path)?;

        // Commits the changelog file to the repository
        self.commit_changelog(&user)?;
        Ok(())
    }

    /// Builds a new changelog plugin instance based on a given list of verified commits.
    ///
    /// Maps the commit types (the key) to a vector of commit messages (the value) for the sections
    /// field of a [ChangelogPlugin]. Therefore, the key contains a vector of commit messages with
    /// the same type, e.g. :
    /// BTreeMap<[ReleaseAction::Major, ["break: breaking change", "break(github): a change"]],
    ///          [ReleaseAction::Minor, ["feat: some feature", "feat(github): another feature"]],
    ///          [ReleaseAction::Patch, ["refac: add comments"]]>
    fn with_commits(&mut self, commits: Vec<Commit>, last_tag: &str, new_tag: &str, repo_url: &str) -> &Self {
        self.last_tag = last_tag.into();
        self.new_tag = new_tag.into();
        self.repo_url = repo_url.into();
        let mut commits_major: Vec<Commit> = vec![];
        let mut commits_minor: Vec<Commit> = vec![];
        let mut commits_patch: Vec<Commit> = vec![];

        for commit in &commits {
            match commit.release_action {
                Some(ReleaseAction::Major) => commits_major.push(commit.clone()),
                Some(ReleaseAction::Minor) => commits_minor.push(commit.clone()),
                Some(ReleaseAction::Patch) => commits_patch.push(commit.clone()),
                None => continue,
            }
        }

        self.sections.insert(ReleaseAction::Major, commits_major);
        self.sections.insert(ReleaseAction::Minor, commits_minor);
        self.sections.insert(ReleaseAction::Patch, commits_patch);

        self
    }

    /// Saves the changelog file contents to the filesystem
    ///
    /// This function writes the changelog file to a provided path from a configuration file.
    /// It creates the file if it doesn't exist. It appends the new log to the file if it already exists.
    /// The log are written in a reverse chronological order, hence the most recent at the top.
    fn serialize(&self, changelog_path: &Path) -> ChangelogResult<()> {
        // Loads the path from the configuration file
        let path = Path::new(changelog_path);

        let mut file: File;
        // Creates a buffer to keep the existing changelog
        let mut buffer = String::new();

        match path.try_exists() {
            Ok(true) => {
                // The file exists: reads it and stores its content.
                file = File::open(path)?;
                file.read_to_string(&mut buffer)?;
            }
            Ok(false) => {
                // The file doesn't exist: creates directory according the provided path.
                create_dir_all(path.parent().unwrap_or(Path::new(CHANGELOG_DEFAULT_PATH)))?;
            }
            Err(err) => return Err(ChangelogError::IoError(err)),
        }

        // Creates or opens the file (overwrite mod) to write on it
        file = File::create(path)?;

        // Writes the new tag and its link to compare repository between new and last tags
        let version_tag = format!(
            "[{}]({}/compare/{}..{})",
            self.new_tag, self.repo_url, self.last_tag, self.new_tag,
        );
        let version_text = format!("## {version_tag}");

        // Creates the date like `2023-02-01`
        let now = OffsetDateTime::now_utc();
        let date_format = format_description::parse("[year]-[month]-[day]")?;
        let date = now.format(&date_format)?;

        // Writes the tag, its compare link and the date as header.
        writeln!(&mut file, "{version_text} ({date})\n",)?;

        // Loops over [ChangelogPlugin]'s sections field to write the file
        for (release_action, commits) in &self.sections {
            if !commits.is_empty() {
                match release_action {
                    ReleaseAction::Major => writeln!(&mut file, "* **Major changes**")?,
                    ReleaseAction::Minor => writeln!(&mut file, "* **Minor changes**")?,
                    ReleaseAction::Patch => writeln!(&mut file, "* **Patch changes**")?,
                }
            }

            for commit in commits {
                let hash = &commit.hash;
                let link = format!("{}/commit/{}", self.repo_url, hash);
                writeln!(&mut file, " * {} ([{}]({}))", commit.message, &commit.hash[0..8], link)?;
            }
        }

        writeln!(&mut file, "\n")?;

        // Appends the previous changelog to the file
        file.write_all(buffer.as_bytes())?;

        Ok(())
    }

    /// Commits the new changelog file and the new tag
    ///
    /// This function commits the file to the repository with the provided path.
    /// The commit message is like `Release v3.2.1`.
    fn commit_changelog(&self, user: &RepositoryUser) -> ChangelogResult<()> {
        let commit_user = format!(r#"git config user.name "{}""#, user.name);
        let commit_user_email = format!(r#"git config user.email "{}""#, user.email);
        let commit_message = format!(r#"git commit -m "Release {}""#, &self.new_tag);

        Command::new("sh")
            .args(["-c", commit_user.as_str()])
            .status()
            .expect("Failed");

        Command::new("sh")
            .args(["-c", commit_user_email.as_str()])
            .status()
            .expect("Failed");

        Command::new("sh").args(["-c", "git add -A"]).status().expect("Failed");

        Command::new("sh")
            .args(["-c", commit_message.as_str()])
            .status()
            .expect("Failed");

        Command::new("sh").args(["-c", "git push"]).status().expect("Failed");

        Ok(())
    }
}

#[cfg(test)]
mod tests;

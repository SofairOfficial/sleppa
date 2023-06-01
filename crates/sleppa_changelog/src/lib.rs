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
//! While the file is written, it has to be automatically commited to the repository with a message : `Release v4.0.0`
//! where `v4.0.0` is the new tag.
//!
//! Datas used to create the changelog are retrieved from a [Context] structure.
//! The plugin loads [CONTEXT_LAST_TAG], [CONTEXT_NEW_TAG], [CONTEXT_USER] and [CONTEXT_COMMITS] from the [Context].
//! Therefore these keys/values must be provided.
//!
//! The changelog file is written, by default, at the [CHANGELOG_DEFAULT_PATH].
//! However, another path could be provided thanks to the method `with_configuration()` to set the path and the name of the file.

pub mod constants;
mod errors;

use constants::CHANGELOG_DEFAULT_PATH;
use errors::{ChangelogError, ChangelogResult};
use sleppa_primitives::{
    repositories::{GitRepository, RepositoryUser},
    Context, {Commit, ReleaseAction},
};
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use time::{format_description, OffsetDateTime};

/// Definition of the Changelog plugin.
///
/// Changelog structure contains mandatory elements to create the file, namely, the map between commit type and
/// commit messages, the last tag, the new tag and the URL of the repository.
/// The URL is used to write hlink in the changelog file, therefore using a String here is sufficient.
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

    changelog_path: Option<String>,
}

impl Default for ChangelogPlugin {
    /// Provides the implementation of the Default trait.
    ///
    /// The changelog_path field is set to `None` to fallback to the default changelog file path
    fn default() -> Self {
        Self {
            sections: BTreeMap::new(),
            last_tag: String::new(),
            new_tag: String::new(),
            repo_url: String::new(),
            changelog_path: None,
        }
    }
}

impl ChangelogPlugin {
    /// Implementation of the `new` method : `ChangelogPlugin::new()`.
    pub fn new() -> Self {
        ChangelogPlugin::default()
    }

    /// Provides a new changelog file path to the ChangelogPlugin
    ///
    /// The default path and name are [CHANGELOG_DEFAULT_PATH], however another one can be set with this method.
    /// ChangelogPlugin::new().with_configuration("mydir/mychangelog.md");
    pub fn with_configuration(&mut self, file_path: &str) -> &mut Self {
        self.changelog_path = Some(file_path.to_string());
        self
    }

    /// Executes the main function of the changelog generator plugin
    ///
    /// This function builds the [ChangelogPlugin] from a vector of [Commit]s contained in a [Context]
    /// and writes the file to a provided path or, by default, to the [CHANGELOG_DEFAULT_PATH].
    /// The file is written using the commits messages as source of information. The changelog groups the
    /// commits using their [ReleaseAction] type.
    pub fn run<R: GitRepository>(&mut self, context: &Context<R>) -> ChangelogResult<()> {
        let commits = match context.load_commits() {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No commits found.".to_string())),
        };

        let last_tag = match context.load_last_tag() {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No last tag found.".to_string())),
        };

        let new_tag = match context.load_new_tag() {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("No new tag found.".to_string())),
        };

        let user = match context.load_user() {
            Some(value) => value,
            None => return Err(ChangelogError::InvalidContext("Missing user".to_string())),
        };

        let repo_url = context.repository.get_url();

        // Builds the [ChangelogPlugin] from commits, last tag and new tag
        self.with_commits(
            commits,
            last_tag.identifier.as_str(),
            new_tag.identifier.as_str(),
            repo_url.as_str(),
        );

        // Verifies if the user provided a changelog file path.
        // Fallback to the [CHANGELOG_DEFAULT_PATH] if none.
        let file_path = match &self.changelog_path {
            Some(path) => Path::new(path),
            None => Path::new(CHANGELOG_DEFAULT_PATH),
        };

        // Creates the changelog file at the given path
        self.serialize(file_path)?;

        // Commits the changelog file to the repository
        self.commit_changelog(&user, file_path)?;
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
    fn commit_changelog(&self, user: &RepositoryUser, changelog_path: &Path) -> ChangelogResult<()> {
        let commit_user = format!(r#"git config user.name "{}""#, user.name);
        let commit_user_email = format!(r#"git config user.email "{}""#, user.email);
        let commit_message = format!(r#"git commit -m "Release {}""#, &self.new_tag);
        let git_add = format!(r#"git add {}"#, changelog_path.to_string_lossy());

        Command::new("sh").args(["-c", commit_user.as_str()]).status()?;

        Command::new("sh").args(["-c", commit_user_email.as_str()]).status()?;

        Command::new("sh").args(["-c", git_add.as_str()]).status()?;

        Command::new("sh").args(["-c", commit_message.as_str()]).status()?;

        Command::new("sh").args(["-c", "git push"]).status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;

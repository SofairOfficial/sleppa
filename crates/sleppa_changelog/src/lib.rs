//! Sleppa changelog management package
//!
//! This package aims to generate the changelog file of a reporistory according the last tag
//! and its associated commits.
//!
//! The commits will be load and according to their message, different sections in the file
//! will be written.
//! These sections represent the type of commit.
//!
//! The changelog file looks like :
//!
//!```
//! ### [v4.0.0](https://github.com/user/repo/compare/v3.2.1..v4.0.0) (2023-05-05)
//!
//! * **break**
//!  * new breaking ([1ebdf43e](https://github.com/user/repo/commit/1ebdf43e8950d8f9dace2e554be5d387267575ef))
//! * **feat**
//!  * new feature ([172cd158](https://github.com/user/repo/commit/172cd1589d0a29b56cd8261a888911201305b04d))
//! * **patch**
//!  * new patch ([cd2fe770](https://github.com/user/repo/commit/cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))
//!```
//!
//! While the file is written, it has to be automatically commited to the reposiroty with a message : `Release v4.0.0`.
//!
//!
//!

mod errors;

use errors::{ChangelogError, ChangelogResult};
use std::collections::BTreeMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;
use time::{format_description, OffsetDateTime};

/// Define the Changelog with its fields.
///
/// Changelog structure contains mandatory elements to create the file, namely, the map between commit type and
/// commit messages, the last tag, the new tag and the URL of the repository.
#[derive(Default)]
pub struct Changelog {
    /// Sections is the commit's type (the keys) associated with their [Commit]s (the value).
    pub sections: BTreeMap<String, Vec<Commit>>,
    /// The reposiroty's previous tag
    pub last_tag: String,
    /// The repository's new tag
    pub new_tag: String,
    /// The repository's URL
    pub repo_url: String,
}

/// Defines Commit and its fields used for the changelog
#[derive(Debug, Clone)]
pub struct Commit {
    /// The 40 char hash
    pub hash: String,
    /// The commit message
    pub message: String,
    /// The commit type
    pub commit_type: String,
}

/// Defines the contracts between a configuration and the changelog.
pub trait ChangelogConfiguration {
    /// Retrieves the path to the changelog file.
    ///
    /// Default path is `./changelogs/CHANGELOG.md`
    fn load_changelog_configuration(&self) -> &Path {
        let default_path = Path::new("./changelogs/CHANGELOG.md");
        default_path
    }
}

/// Defines the contract to bring analyzed commit to changelog.
pub trait ChangelogCommit {
    /// Brings the verified commits since the last tag.
    fn load_changelog_commits(&self) -> Vec<Commit>;
}

impl Changelog {
    /// Implementation of the `new` method : `ChangelogMap::new()`
    pub fn new() -> Self {
        Changelog::default()
    }

    /// Creates a new Changelog from verified commits
    ///
    /// Maps the commit types (the key) to a vector of commit messages (the value) for the section
    /// field of a [Changelog]. Therefore, the key value contains a vector of commit messages with
    /// the same type.
    pub fn new_from_commits<C>(commits_to_analyze: C) -> Self
    where
        C: ChangelogCommit,
    {
        let mut changelog_map = Changelog::new();
        // Brings the verified commits
        let commits = commits_to_analyze.load_changelog_commits();

        for commit in &commits {
            if changelog_map.sections.contains_key(&commit.commit_type) {
                // If the key exists, append the commit to the value.
                let mut existing_value = changelog_map.sections[&commit.commit_type].clone();
                existing_value.push(commit.clone());

                changelog_map
                    .sections
                    .insert(commit.commit_type.to_string(), existing_value);
            } else {
                // If the key doesn't exists, creates it and add the commit to the value.
                changelog_map
                    .sections
                    .insert(commit.commit_type.to_string(), vec![commit.clone()]);
            }
        }
        changelog_map
    }

    /// Writes the changelog file
    ///
    /// This function writes the changelog file to a provided path from a configuration file.
    /// It creates the file id it doesn't exist. It appends the new log to the file if it already exists.
    /// The log are written in a inverse chronological order, hence the most recent at the top.
    pub fn write_changelog<P>(&self, config: P) -> ChangelogResult<()>
    where
        P: ChangelogConfiguration,
    {
        // Loads the path from the configuration file
        let path = config.load_changelog_configuration();

        let mut file: File;
        // Creates a buffer to keep the existing changelog
        let mut buffer = String::new();

        // TOCTOU Attacks possible / dangerous here?
        match path.try_exists() {
            Ok(true) => {
                file = File::open(path)?;
                file.read_to_string(&mut buffer)?;
            }
            Ok(false) => {
                create_dir_all(path.parent().unwrap_or(Path::new("changelogs/")))?;
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

        // Writes the tag, its link and the date as header.
        writeln!(&mut file, "{version_text} ({date})\n",)?;

        // Loops over [Changelog]'s section field to write the file
        for (commit_type, commits) in &self.sections {
            writeln!(&mut file, "* **{commit_type}**")?;

            for entry in commits {
                let hash = &entry.hash;
                let link = format!("{}/commit/{}", self.repo_url, hash);
                write!(&mut file, " * {} ([{}]({}))\n", entry.message, &entry.hash[0..8], link)?;
            }
        }

        writeln!(&mut file, "\n\n")?;

        // Appends the previous changelog to the file
        file.write(buffer.as_bytes())?;

        Ok(())
    }
}

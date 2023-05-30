/// This module regroups all the constants used in the `sleppa_changelog` crate.

/// The key for `sleppa_changelog` in the `Context` to acces sleppa_changelog's `configuration`.
pub const CHANGELOG_KEY: &str = "sleppa_changelog";

/// The key to access the user defined path for the changelog file in the configuration.
pub const CHANGELOG_FILE_KEY: &str = "changelog_file_path";

/// The key to access the repository url in the configuration.
pub const REPOSIROTY_URL_KEY: &str = "repository_url";

/// The default path for the changelog file.
pub const CHANGELOG_DEFAULT_PATH: &str = "changelogs/CHANGELOG.md";

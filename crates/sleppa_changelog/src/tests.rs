//! Unit tests
//!
//! This testing module implements the unit tests for testing the changelog generator routines.

use super::*;
use rstest::*;
use tempfile::tempdir;

/// Use fixture to create a reusable component
#[fixture]
fn commits_constructor() -> Vec<Commit> {
    // Creates multiple commit
    // break type
    let commit1 = Commit {
        message: "break: new breaking".to_string(),
        commit_type: "break".to_string(),
        hash: "1ebdf43e8950d8f9dace2e554be5d387267575ef".to_string(),
    };

    // feat type
    let commit2_1 = Commit {
        message: "feat: new feature".to_string(),
        commit_type: "feat".to_string(),
        hash: "172cd1589d0a29b56cd8261a888911201305b04d".to_string(),
    };
    let commit2_2 = Commit {
        message: "feat: another feature".to_string(),
        commit_type: "feat".to_string(),
        hash: "000cd1589d0a29b56cd8261a888911201305b04d".to_string(),
    };

    // patch type
    let commit3_1 = Commit {
        message: "patch: new patch".to_string(),
        commit_type: "patch".to_string(),
        hash: "cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a".to_string(),
    };
    let commit3_2 = Commit {
        message: "patch: another patch".to_string(),
        commit_type: "patch".to_string(),
        hash: "000fe77015b7aa2ac666ec05e14b76c9ba3dfd0a".to_string(),
    };
    let commit3_3 = Commit {
        message: "patch: also a patch".to_string(),
        commit_type: "patch".to_string(),
        hash: "111fe77015b7aa2ac666ec05e14b76c9ba3dfd0a".to_string(),
    };

    // Constructs the vector of commits
    let commits = vec![commit1, commit2_1, commit2_2, commit3_1, commit3_2, commit3_3];
    commits
}

// Tests the method `build_from_commits`.
//
// The method must return a &[ChangelogPlugin] with convenient provided fields.
#[rstest]
fn test_can_build_from_commit(commits_constructor: Vec<Commit>) -> TestResult<()> {
    // Unit test preparation
    let last_tag = "v3.2.1";
    let new_tag = "v4.0.0";
    let repo_url = "https://github.com/user/repo";

    let commits = commits_constructor;
    let mut changelog_plugin = ChangelogPlugin::new();

    let mut sections = BTreeMap::new();
    sections.insert("break".to_string(), vec![commits[0].clone()]);
    sections.insert("feat".to_string(), vec![commits[1].clone(), commits[2].clone()]);
    sections.insert(
        "patch".to_string(),
        vec![commits[3].clone(), commits[4].clone(), commits[5].clone()],
    );

    // Execution step
    changelog_plugin.build_from_commits(commits, last_tag, new_tag, repo_url);

    // Asserts the builded [ChangelogPlugin] is correct
    assert_eq!(changelog_plugin.last_tag, last_tag);
    assert_eq!(changelog_plugin.new_tag, new_tag);
    assert_eq!(changelog_plugin.repo_url, repo_url);
    debug_assert_eq!(changelog_plugin.sections, sections);

    Ok(())
}

// Tests the method `execute` with a new file.
//
// The method returns a [ChangelogResult] and creates then writes a changelog file.
#[rstest]
fn test_can_serialize_file_exists(commits_constructor: Vec<Commit>) -> TestResult<()> {
    // Unit test preparation
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("changelogs");
    file_path.join("CHANGELOG").set_extension("md");

    let last_tag = "v3.2.1";
    let new_tag = "v4.0.0";
    let repo_url = "https://github.com/user/repo";

    let commits = commits_constructor;
    let mut changelog_plugin = ChangelogPlugin::new();

    changelog_plugin.build_from_commits(commits, last_tag, new_tag, repo_url);

    changelog_plugin.serialize(&file_path)?;

    // Reads the file to assert equality
    let mut buffer = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut buffer)?;

    // Creates the date like `2023-02-01`
    let now = OffsetDateTime::now_utc();
    let date_format = format_description::parse("[year]-[month]-[day]")?;
    let date = now.format(&date_format)?;

    let test_file = format!("## [v4.0.0](https://github.com/user/repo/compare/v3.2.1..v4.0.0) ({date})") + "\n\n" +
        "* **break**\n" + 
        " * break: new breaking ([1ebdf43e](https://github.com/user/repo/commit/1ebdf43e8950d8f9dace2e554be5d387267575ef))\n" +
        "* **feat**\n" +
        " * feat: new feature ([172cd158](https://github.com/user/repo/commit/172cd1589d0a29b56cd8261a888911201305b04d))\n" +
        " * feat: another feature ([000cd158](https://github.com/user/repo/commit/000cd1589d0a29b56cd8261a888911201305b04d))\n" +
        "* **patch**\n" +
        " * patch: new patch ([cd2fe770](https://github.com/user/repo/commit/cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n" +
        " * patch: another patch ([000fe770](https://github.com/user/repo/commit/000fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n" +
        " * patch: also a patch ([111fe770](https://github.com/user/repo/commit/111fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n\n\n";

    // Asserts equality
    assert_eq!(test_file, buffer);

    Ok(())
}

// Tests the method `execute` with an existing changelog file.
//
// The method returns a [ChangelogResult] and writes a changelog file into one already existing.
// It appends the text to the new changelog file in a reverse chronological order.
#[rstest]
fn test_can_serialize_file_not_exist(commits_constructor: Vec<Commit>) -> TestResult<()> {
    // Unit test preparation
    let tmp_dir = tempdir()?;
    let file_path = tmp_dir.path().join("changelogs");
    file_path.join("CHANGELOG").set_extension("md");

    // Creates the file and writes something in it
    let mut file = File::create(&file_path)?;
    writeln!(&mut file, "This is a changelog file")?;

    let last_tag = "v3.2.1";
    let new_tag = "v4.0.0";
    let repo_url = "https://github.com/user/repo";

    let commits = commits_constructor;
    let mut changelog_plugin = ChangelogPlugin::new();

    changelog_plugin.build_from_commits(commits, last_tag, new_tag, repo_url);

    changelog_plugin.serialize(&file_path)?;

    // Reads the file to assert equality
    let mut buffer = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut buffer)?;

    // Creates the date like `2023-02-01`
    let now = OffsetDateTime::now_utc();
    let date_format = format_description::parse("[year]-[month]-[day]")?;
    let date = now.format(&date_format)?;

    let test_file = format!("## [v4.0.0](https://github.com/user/repo/compare/v3.2.1..v4.0.0) ({date})") + "\n\n" +
        "* **break**\n" + 
        " * break: new breaking ([1ebdf43e](https://github.com/user/repo/commit/1ebdf43e8950d8f9dace2e554be5d387267575ef))\n" +
        "* **feat**\n" +
        " * feat: new feature ([172cd158](https://github.com/user/repo/commit/172cd1589d0a29b56cd8261a888911201305b04d))\n" +
        " * feat: another feature ([000cd158](https://github.com/user/repo/commit/000cd1589d0a29b56cd8261a888911201305b04d))\n" +
        "* **patch**\n" +
        " * patch: new patch ([cd2fe770](https://github.com/user/repo/commit/cd2fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n" +
        " * patch: another patch ([000fe770](https://github.com/user/repo/commit/000fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n" +
        " * patch: also a patch ([111fe770](https://github.com/user/repo/commit/111fe77015b7aa2ac666ec05e14b76c9ba3dfd0a))\n\n\n" +
        "This is a changelog file\n"; // This is the text already in the file

    // Asserts equality
    assert_eq!(test_file, buffer);

    Ok(())
}

// Unit test result type
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
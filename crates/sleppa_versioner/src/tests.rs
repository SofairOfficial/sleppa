//! Unit tests
//!
//! This testing module implements the unit tests for versioning.
use super::{errors::*, *};

// Tests the conversion of a string tag into a [Tag] structure
#[test]
fn test_can_try_into() -> TestResult<()> {
    // Unit test preparation
    let good_tag = "v3.2.1";
    let bad_tag = "v3.1";

    // Execution step
    let tag_as_tagstruct: Tag = Tag::try_from(good_tag)?;

    // Asserts the tag is correctly converted with a good tag
    assert_eq!(
        tag_as_tagstruct,
        Tag {
            major: 3,
            minor: 2,
            patch: 1
        }
    );

    // Asserts a error occurs with a bad tag
    assert!(Tag::try_from(bad_tag).is_err());

    Ok(())
}

// Tests the parsing from Tag to String
#[test]
fn test_can_into_string() {
    // Unit test preparation
    let tag = Tag {
        major: 3,
        minor: 2,
        patch: 1,
    };

    let tag_string: String = tag.into();

    assert_eq!(tag_string, "v3.2.1");
}

// Tests a Tag's incrementation from a release action type
#[test]
fn test_can_increment() {
    // Unit test preparation
    let tag = Tag {
        major: 3,
        minor: 2,
        patch: 1,
    };

    // Execution step
    let new_tag_major = tag.increment(&ReleaseAction::Major);
    let new_tag_minor = tag.increment(&ReleaseAction::Minor);
    let new_tag_patch = tag.increment(&ReleaseAction::Patch);

    // Asserts incrementation are correct
    assert_eq!(
        new_tag_major,
        Tag {
            major: 4,
            minor: 0,
            patch: 0,
        }
    );

    assert_eq!(
        new_tag_minor,
        Tag {
            major: 3,
            minor: 3,
            patch: 0,
        }
    );

    assert_eq!(
        new_tag_patch,
        Tag {
            major: 3,
            minor: 2,
            patch: 2,
        }
    );
}

//! Sleppa primitives package
//!
//! This crate provides convenient primitives, like, structures, modules or
//! reusable code.

pub mod repositories;

/// Defines Commit and its fields used for the changelog
#[derive(Debug, Clone, PartialEq)]
pub struct Commit {
    /// long commit identifier (i.e. 40 digits long SHA-1 hash)
    pub hash: String,
    /// The commit message like: `feat(github): a new feature`
    pub message: String,
    /// Commit message type value, e.g. `feat`, `break`, `refac`, etc.
    pub commit_type: String,
}

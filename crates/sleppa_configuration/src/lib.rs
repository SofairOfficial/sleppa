//! Sleppa configuration management package
//!

use serde::{Deserialize, Serialize};

/// Configuration data structure
///
/// This structure will be used to deserialize the toml into this Rust usable type.
///
/// The `release_rules` hashmap contains 3 keys : `major`, `minor` and `patch`.
/// For every key a [ReleaseRule] is associated.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configuration;

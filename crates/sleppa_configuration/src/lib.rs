//! Sleppa configuration management package
//!
//! A [Configuration] is a structure used to store value used by crates. It contains a field `map` which
//! is defined by a [Hashmap].
//!
//! To access a specific crate's [Configuration], a key is defined in a `constants` module in the crate.
//! Then datas are retrieved with a key also defined in the constants module.

pub mod errors;

use sleppa_primitives::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub map: HashMap<String, Value>,
}

impl Configuration {
    /// Loads an optionnal [RepositoryUser] from the context
    pub fn load(&self, key: &str) -> Value {
        self.map[key].clone()
    }
}

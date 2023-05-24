//! Sleppa configuration management package
//!

pub mod constants;
pub mod errors;

use sleppa_primitives::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub map: HashMap<String, Value>,
}

/// The context structure is used to store configuration.
#[derive(Default)]
pub struct Context {
    pub configurations: HashMap<String, Configuration>,
}

/// If the plugin needs a configuration to work, this traits defines the behavior to load this
/// configuration
pub trait Configurable<T> {
    fn load(context: &Context) -> T;
}

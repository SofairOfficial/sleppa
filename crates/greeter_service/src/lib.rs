//! Greeter service library
//!
//! This module implements a tiny greeter service for testing various tools that are pre-configured
//! in this Rust project template.
//!
//! ## Example
//! ```
//! use greeter_service::{GreeterInterface, GreeterService};
//!
//! let service = GreeterService::new("SampleGreeter");
//! println!("Result: {}", service.greet("Jack"));
//! ```

// Declare and re-export greeter service modules
pub mod service;
pub use service::*;

pub mod traits;
pub use traits::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet() {
        let service = GreeterService::new("MyService");
        assert_eq!(service.greet("Jonathan"), "[MyService] - Hello and welcome: Jonathan")
    }
}

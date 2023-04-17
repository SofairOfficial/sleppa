//! Greeter mocking
//!
//! This crate implements a tiny greeter service mock using the crate
//! 'mockall' (https://github.com/asomers/mockall)
//!
//! The use case scenario is the following :
//! You know how the trait 'GreeterInterface' is implemented. It is used with a struct
//! or trait like 'GreeterService'.
//! This struct or trait is not provided right now. So we mock it.

use std::env;
use std::process::exit;

pub mod traits;
use traits::GreeterInterface;

use mockall::*;

fn main() {
    // Process command-line arguments
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() <= 1 {
        println!("Who would you like to greet? Please give a name!");
        println!("\nUsage: greetermock [name]");
        exit(1);
    }

    // Call greeter service (first command-line argument is the name)
    // You mock the trait here with a mutable initialization
    let mut service = MockGreeterService::new();

    // The 'greet' function behavior is defined here and can be called after
    service
        .expect_greet()
        .returning(|who: &str| format!("Hello and welcome: {}", who));
    println!("\n{:?}", service.greet(&arguments[1]));
}

// The GreeterService struct or trait is not provided. The mock takes place here.
mock! {
    GreeterService {}

    impl GreeterInterface for GreeterService{
        fn greet(self, who: &str) -> String;
    }
}

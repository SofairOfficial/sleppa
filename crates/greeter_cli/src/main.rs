use std::env;
use std::process::exit;

use greeter_service::{GreeterInterface, GreeterService};

fn main() {
    // Process command-line arguments
    let arguments: Vec<_> = env::args().collect();
    if arguments.len() <= 1 {
        println!("Who would you like to greet? Please give a name!");
        println!("\nUsage: greeter [name]");
        exit(1);
    }

    // Call greeter service (first command-line argument is the name)
    let service = GreeterService::new("SampleGreeter");
    println!("\n{}", service.greet(&arguments[1]));
}

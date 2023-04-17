use crate::traits::GreeterInterface;

/// Greeter service structure
pub struct GreeterService {
    /// Name of the service
    name: &'static str,
}

// Implement greeter service interface
impl GreeterInterface for GreeterService {
    fn greet(self, who: &str) -> String {
        format!("[{}] - Hello and welcome: {}", self.name, who)
    }
}

// Implement greeter service function
impl GreeterService {
    /// Builds a new greeter service instance
    pub fn new(name: &'static str) -> Self {
        GreeterService { name }
    }
}

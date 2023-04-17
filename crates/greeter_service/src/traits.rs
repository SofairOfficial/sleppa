/// Greeter service interface (or contract)
pub trait GreeterInterface {
    /// Display a greeting mesage to the sender with the given name
    fn greet(self, name: &str) -> String;
}

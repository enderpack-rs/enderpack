pub mod sort;
pub use sort::*;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Argument<T: Display> {
    name: String,
    value: T,
}

impl<T: Display> Argument<T> {
    pub fn new(name: &str, value: T) -> Self {
        Argument {
            name: name.to_owned(),
            value,
        }
    }
}

impl<T: Display> Display for Argument<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

use std::fmt::Display;

#[derive(Debug)]
pub struct Namespace(String);

impl Namespace {
    pub fn new(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

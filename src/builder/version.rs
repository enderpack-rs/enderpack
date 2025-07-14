use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Version(u32);

impl Version {
    pub fn new(version_number: u32) -> Version {
        Version(version_number)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::fmt::Display;

use regex::{Error, Regex};

pub struct Objective(String);

impl Objective {
    pub fn new(name: &str) -> Result<Objective, Error> {
        let regex = Regex::new("^[-+._A-Za-z0-9]+$")?;
        if regex.is_match(name) {
            Ok(Objective(name.to_owned()))
        } else {
            Err(regex::Error::Syntax(name.to_owned()))
        }
    }
}

impl Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

use std::fmt::Display;

use super::Selector;

#[derive(Clone)]
pub struct PlayerSelector(String);

impl PlayerSelector {
    pub fn new(name: &str) -> PlayerSelector {
        PlayerSelector(name.to_owned())
    }
}

impl Selector for PlayerSelector {}

impl Display for PlayerSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

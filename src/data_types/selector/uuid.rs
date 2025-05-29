use std::fmt::Display;

use uuid::Uuid;

use super::Selector;

pub struct UUIDSelector(Uuid);

impl UUIDSelector {
    pub fn new(uuid: Uuid) -> UUIDSelector {
        UUIDSelector(uuid)
    }
}

impl Selector for UUIDSelector {}

impl Display for UUIDSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.hyphenated())
    }
}

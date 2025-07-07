use std::{borrow::Borrow, fmt::Display};

use crate::prelude::Selector;

use super::Command;

pub struct Tellraw<T: Selector> {
    selector: T,
    message: serde_json::Value,
}

pub fn tellraw<T: Selector>(
    selector: impl Borrow<T>,
    message: impl Borrow<serde_json::Value>,
) -> Tellraw<T> {
    Tellraw {
        selector: selector.borrow().clone(),
        message: message.borrow().clone(),
    }
}

impl<T: Selector> Command for Tellraw<T> {}

impl<T: Selector> Display for Tellraw<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tellraw {} {}", self.selector, self.message)
    }
}

use crate::data_types::selector::Selector;

use super::Command;

pub struct Effect {
    selector: Selector,
}

impl Command for Effect {
    fn into_mcfunction(self) -> String {
        format!("effect {}", self.selector)
    }
}

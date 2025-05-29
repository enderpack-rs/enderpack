use std::fmt::Display;

use derive_new::new;

use crate::data_types::{
    range::MCRange,
    selector::{argument::Argument, variable::SelectorVariable},
};

use super::Selector;

#[derive(new)]
pub struct TargetSelector {
    variable: SelectorVariable,
    #[new(default)]
    distance: Option<Argument<MCRange>>,
}

impl TargetSelector {
    pub fn all() -> Self {
        Self::new(SelectorVariable::E)
    }
    pub fn all_players() -> Self {
        Self::new(SelectorVariable::A)
    }
    pub fn distance<T>(mut self, distance: T) -> Self
    where
        MCRange: std::convert::From<T>,
    {
        self.distance = Some(Argument::new("distance", MCRange::from(distance)));
        self
    }
}

impl Selector for TargetSelector {}

impl Display for TargetSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sel = String::new();
        match self.variable {
            SelectorVariable::P => sel.push_str("@p"),
            SelectorVariable::R => sel.push_str("@r"),
            SelectorVariable::A => sel.push_str("@a"),
            SelectorVariable::E => sel.push_str("@e"),
            SelectorVariable::S => sel.push_str("@s"),
            SelectorVariable::N => sel.push_str("@n"),
        }
        let mut args: Vec<String> = Vec::new();
        for arg in [&self.distance].into_iter().flatten() {
            // e.g. <argument>=<value>
            args.push(arg.to_string())
        }
        if !args.is_empty() {
            sel.push('[');
            sel.push_str(
                args.iter()
                    .fold(String::new(), |acc, arg| format!("{arg}, {acc}"))
                    .as_str()
                    .trim_end_matches(", "),
            );
            sel.push(']');
        }
        write!(f, "{}", sel)
    }
}

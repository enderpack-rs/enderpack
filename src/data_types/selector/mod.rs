pub mod argument;
pub mod target;

use crate::data_types::range::*;
use crate::data_types::selector::{argument::*, target::*};
use derive_builder::*;
use std::{fmt::Display, ops::Range};

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Selector {
    target: Target,
    #[builder(setter(custom))]
    distance: Argument<MCRange>,
}

impl SelectorBuilder {
    pub fn distance(mut self, distance: Range<f64>) -> Self {
        self.distance = Some(Argument::new("distance", MCRange::Bound(distance)));
        self
    }
}

impl Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sel = String::new();
        match self.target {
            Target::P => sel.push_str("@p"),
            Target::R => sel.push_str("@r"),
            Target::A => sel.push_str("@a"),
            Target::E => sel.push_str("@e"),
            Target::S => sel.push_str("@s"),
            Target::N => sel.push_str("@n"),
        }
        let mut args: Vec<String> = Vec::new();
        for arg in [&self.distance].into_iter() {
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

pub fn all() -> SelectorBuilder {
    SelectorBuilder::default().target(Target::E)
}

pub fn all_players() -> SelectorBuilder {
    SelectorBuilder::default().target(Target::A)
}

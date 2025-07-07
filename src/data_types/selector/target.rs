use std::fmt::Display;

use derive_new::new;

use crate::data_types::{
    range::MCRange,
    selector::{argument::*, variable::SelectorVariable},
};

use super::Selector;

#[derive(new, Clone)]
pub struct TargetSelector {
    variable: SelectorVariable,
    #[new(default)]
    distance: Option<Argument<MCRange>>,
    #[new(default)]
    limit: Option<Argument<u32>>,
    #[new(default)]
    sort: Option<Argument<Sort>>,
    #[new(default)]
    x: Option<Argument<f64>>,
    #[new(default)]
    y: Option<Argument<f64>>,
    #[new(default)]
    z: Option<Argument<f64>>,
    #[new(default)]
    dx: Option<Argument<f64>>,
    #[new(default)]
    dy: Option<Argument<f64>>,
    #[new(default)]
    dz: Option<Argument<f64>>,
}

macro_rules! setter {
    ($name:ident, $type:ty) => {
        pub fn $name(mut self, $name: $type) -> Self {
            self.$name = Some(Argument::new("$name", $name));
            self
        }
    };
}

pub fn all() -> TargetSelector {
    TargetSelector::new(SelectorVariable::E)
}
pub fn all_players() -> TargetSelector {
    TargetSelector::new(SelectorVariable::A)
}

impl TargetSelector {
    pub fn distance<T>(mut self, distance: T) -> Self
    where
        MCRange: std::convert::From<T>,
    {
        self.distance = Some(Argument::new("distance", MCRange::from(distance)));
        self
    }
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(Argument::new("limit", limit));
        self
    }
    pub fn xyz(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = Some(Argument::new("x", x));
        self.y = Some(Argument::new("y", y));
        self.z = Some(Argument::new("z", z));
        self
    }
    setter!(sort, Sort);
    setter!(x, f64);
    setter!(y, f64);
    setter!(z, f64);
    setter!(dx, f64);
    setter!(dy, f64);
    setter!(dz, f64);
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

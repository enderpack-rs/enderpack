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
    ($fn_name:ident$(<$gen_name:ident: Into<$generic:ident>>)?($($name:ident: $type:ty),+)) => {
        pub fn $fn_name$(<$gen_name: ::std::convert::Into<$generic>>)?(mut self, $($name: $type),+) -> Self {
            $(self.$name = Some(Argument::new("$name", $name.into()));)+
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
    setter!(distance<T: Into<MCRange>>(distance: T));
    setter!(limit(limit: u32));
    setter!(sort(sort: Sort));
    setter!(xyz(x: f64, y: f64, z: f64));
    setter!(x(x: f64));
    setter!(y(y: f64));
    setter!(z(z: f64));
    setter!(dx(dx: f64));
    setter!(dy(dy: f64));
    setter!(dz(dz: f64));
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

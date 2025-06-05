pub mod score;

pub use score::*;

use crate::prelude::Command;

pub trait VariableInit<T> {
    fn new(name: &str, path: &str, value: T) -> Self;
}

pub trait Variable {
    fn get_declaration(&self) -> &impl Command;
    fn get_stack(&self) -> &Vec<Box<dyn Command>>;
}

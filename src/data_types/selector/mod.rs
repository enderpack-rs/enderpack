use std::fmt::Display;

pub mod argument;
pub mod player;
pub mod target;
pub mod uuid;
pub mod variable;

pub use argument::*;
pub use player::*;
pub use target::*;
pub use uuid::*;

pub trait Selector: Display + Clone {}

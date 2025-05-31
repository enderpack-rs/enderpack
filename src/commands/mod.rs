pub mod effect;
pub mod tellraw;
use std::fmt::Display;

pub use effect::*;
pub use tellraw::*;

pub trait Command: Display {}

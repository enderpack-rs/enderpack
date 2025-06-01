pub mod effect;
pub mod scoreboard;
pub mod tellraw;
use std::fmt::Display;

pub use effect::*;
pub use scoreboard::*;
pub use tellraw::*;

pub trait Command: Display {}

#[macro_export]
macro_rules! command_setup {
    ($struct:ident => $function:ident) => {
        pub fn $function() -> $struct {
            $struct {}
        }
    };
}

#[macro_export]
macro_rules! subcommand_setup {
    ($struct:ident {
        $(unit {
            $($unit_c:ident() => $unit_s:ident),*
        };)?
        $(new {
            $($new_c:ident($($new_arg:ident: $new_type:ty),*) => $new_s:ident),*
        };)?
        $(new with $generic:ident {
            $($gen_c:ident($($gen_arg:ident: $gen_type:ty),*) => $gen_s:ident),*
        };)?
    }) => {
        impl $struct {
            $($(
                pub fn $unit_c(self) -> $unit_s {
                    $unit_s {}
                }
            )*)*
            $($(
                pub fn $new_c(self, $($new_arg: $new_type),*) -> $new_s {
                    $new_s::new($($new_arg),*)
                }
            )*)*
            $($(
                pub fn $gen_c<T: $generic>(self, $($gen_arg: $gen_type),*) -> $gen_s<T> {
                    $gen_s::new($($gen_arg),*)
                }
            )*)*
        }
    };
}

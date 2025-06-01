pub mod effect;
pub mod tellraw;
use std::fmt::Display;

pub use effect::*;
pub use tellraw::*;

pub trait Command: Display {}

#[macro_export]
macro_rules! subcommand_setup {
    ($struct:ident => $function:ident { $($subcommand:ident => $substruct:ident),+ }) => {
        impl $struct {
            $(
                pub fn $subcommand(self) -> $substruct {
                    $substruct {}
                }
            )*
        }

        pub fn $function() -> $struct {
            $struct {}
        }
    };
    ($struct:ident { $($subcommand:ident => $substruct:ident),+ }) => {
        impl $struct {
            $(
                pub fn $subcommand(self) -> $substruct {
                    $substruct {}
                }
            )*
        }
    };
}

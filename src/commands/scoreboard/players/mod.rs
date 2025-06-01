use set::ScoreboardPlayersSet;

use crate::{prelude::Selector, subcommand_setup};

pub mod set;

pub struct ScoreboardPlayers;

subcommand_setup!(ScoreboardPlayers {
    new with Selector {
        set(selector: T, objective: &str, score: i32) => ScoreboardPlayersSet
    };
});
